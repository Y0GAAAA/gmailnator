use crate::endpoint::*;
use crate::regexes::MAIL_ID_REGEX;
use crate::http::{UrlQuery, get_response_content};
use crate::errors::GmailnatorError; 

use serde::{Serialize, Deserialize};
use scraper::{Html, Selector};
use htmlescape::decode_html; 
use serde_json::from_str;

//use futures::;

lazy_static! {

    static ref SUBJECT_SELECTOR:Selector = Selector::parse("b").unwrap();
    static ref BODY_SELECTOR:Selector = Selector::parse("div").unwrap();

    static ref BULK_EMAIL_SELECTOR:Selector = Selector::parse("#email-list-message > a").unwrap();

}

/// Default error for the crate.
pub type Error = GmailnatorError;

/// A structure that contains an e-mail subject and its raw content which the `decode_content()` method can decode.
#[derive(Debug, Serialize, Deserialize)]
pub struct MailMessage {
    subject:String,
    raw_content:String,
}

#[derive(Deserialize)]
struct JsonMailMessage {
    subject:String,
    content:String,
}

impl MailMessage {

    pub(crate) fn new(subject:String, raw_content:String) -> Self {
        Self {subject, raw_content}
    }

    pub(crate) fn parse(response_fragment:&str) -> Result<Self, Error> {

        let json_content:JsonMailMessage = match from_str(response_fragment) {

            Ok(message) => message,
            Err(_) => { return Err(Error::JsonParsingError(response_fragment.to_string())); }

        };

        let subject_fragment = Html::parse_fragment(&json_content.subject);
        let content_fragment = Html::parse_fragment(&json_content.content);

        let mut subject_container = subject_fragment.select(&SUBJECT_SELECTOR);
        let mut body_container = content_fragment.select(&BODY_SELECTOR);

        let subject_item = subject_container.next();
        let body_item = body_container.next();

        if subject_item.is_none() {
            return Err(Error::HtmlParsingError(response_fragment.to_string()));
        }

        let is_html_content = body_item.is_none();

        let subject = subject_item.unwrap().inner_html();
        let subject = decode_html(&subject).unwrap_or_default();

        let raw_body = match is_html_content {

            true => json_content.content,
            false => body_item.unwrap().inner_html(),

        };
        
        Ok(MailMessage::new(subject, raw_body))
        
    }

    /// Gets the message's subject.
    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    /// Decodes the raw content's html entities.
    pub fn decode_content(&self) -> Result<String, Error> {
    
        if let Ok(decoded) = decode_html(&self.raw_content) {
            Ok(decoded.to_string())
        } else {
            Err(Error::HtmlDecodingError)
        }
    
    }

    /// Gets the message's raw html content with potential html entities still encoded. 
    pub fn get_raw_content(&self) -> &str {
        &self.raw_content
    }

}

/// The library's main object, when instantiated represents a gmailnator inbox associated to an e-mail address.
#[derive(Debug, Serialize, Deserialize)]
pub struct GmailnatorInbox {

    mail_address:String,    //COMPLETE E-MAIL | Ex : extmp+blabla@gmail.com
    temp_server:String,     //SERVER ID       | Ex : extmp

}

impl GmailnatorInbox {

    const MIN_BULK_COUNT:u32 = 1;
    const MAX_BULK_COUNT:u32 = 1000;

    /// Creates a new inbox. 
    pub fn new() -> Result<Self, Error> {
        
        let email_request = get_request_from_endpoint(GmailnatorEndpoint::GetEmail);
        let mut mail_query = GmailnatorInbox::get_tokened_query();
        
        mail_query.add("action", "GenerateEmail");
        mail_query.add("data%5B%5D", "2");
 
        let response_str = get_response_content(email_request, mail_query)?;

        let server_id = GmailnatorInbox::get_temp_server_id(&response_str)?;

        return Ok(
            Self {
                mail_address:response_str,
                temp_server:server_id,
            }
        );

    }
 
    /// Creates the desired amount of inbox.
    /// The `count` argument must be between 1 and 1000 included. 
    pub fn new_bulk(count:u32) -> Result<Vec<Self>, Error> {

        if count < GmailnatorInbox::MIN_BULK_COUNT 
        || count > GmailnatorInbox::MAX_BULK_COUNT {
            return Err(Error::InvalidCountError(count));
        }

        let bulk_request = get_request_from_endpoint(GmailnatorEndpoint::GetEmailBulk);

        let mut bulk_query = GmailnatorInbox::get_tokened_query();

        bulk_query.add("email_list", &(count - 1).to_string());
        bulk_query.add("email%5B%5D", "2");

        let response_str = get_response_content(bulk_request, bulk_query)?;

        GmailnatorInbox::get_bulk_from_html(&response_str)
        
    }

    /// Creates a new inbox from an  already existing gmailnator address. 
    /// Warning : an invalid gmailnator address will not return an Error.
    /// ```
    /// # use gmailnator::GmailnatorInbox;
    /// let valid   = GmailnatorInbox::from_address("deedtmp+[...]@gmail.com").unwrap();
    /// let invalid = GmailnatorInbox::from_address("invalid.email@gmail.com").unwrap();
    /// ```
    pub fn from_address(address:&str) -> Result<Self, Error> {

        let temp_server_id = GmailnatorInbox::get_temp_server_id(address)?;

        Ok(Self {
            mail_address:address.to_string(),
            temp_server:temp_server_id,
        })

    }

    /// Returns the received e-mail(s) as an iterator.
    /// It's only when calling `next()` on the iterator that the e-mail data will be queried. 
    pub fn get_messages_iter(&self) -> Result<MailMessageIterator, Error> {

        let message_ids = self.get_inbox_messages_id_collection()?;

        let iter = MailMessageIterator {
            message_ids,
            temp_server_identifier:self.temp_server.clone(),
        };

        Ok(iter)

    }

    /// Returns the current inbox e-mail address.
    pub fn get_address(&self) -> &str {
        &self.mail_address
    }

    fn get_bulk_from_html(html:&str) -> Result<Vec<Self>, Error> {

        let document = Html::parse_document(html);
        let mut emails = document.select(&BULK_EMAIL_SELECTOR);

        let mut inbox_list = Vec::<Self>::new();

        while let Some(mail_item) = emails.next() {

            let address = mail_item.text().next().unwrap();
            let inbox_res = Self::from_address(&address);

            if let Ok(inbox) = inbox_res {

                inbox_list.push(inbox);

            }

        }

        Ok(inbox_list)

    }

    fn get_message_by_id(server_identifier:&str, message_id:&str) -> Result<MailMessage, Error> {

        let get_message_request = get_request_from_endpoint(GmailnatorEndpoint::GetMessage);

        let mut get_message_query = GmailnatorInbox::get_tokened_query();

        get_message_query.add("action", "get_message");
        get_message_query.add("message_id", message_id);
        get_message_query.add("email", server_identifier);
        
        let parsable_message = get_response_content(get_message_request, get_message_query)?;

        MailMessage::parse(&parsable_message)

    }

    fn get_tokened_query() -> UrlQuery {

        let mut tokened_query = UrlQuery::new();
        
        tokened_query.add("csrf_gmailnator_token", "");

        tokened_query

    }

    fn get_temp_server_id(mail_address:&str) -> Result<String, Error> {

        let mut server_id = mail_address.split('+');

        if let Some(identifier) = server_id.next() {
            Ok(identifier.to_string())
        } else {
            return Err(Error::MailServerParsingError(mail_address.to_string()));
        }

    }

    fn get_inbox_messages_id_collection(&self) -> Result<Vec<String>, Error> {

        let inbox_request = get_request_from_endpoint(GmailnatorEndpoint::GetInbox);

        let mut query = GmailnatorInbox::get_tokened_query();
        
        query.add("action", "LoadMailList");
        query.add("Email_address", &self.mail_address);

        let response_str = get_response_content(inbox_request, query)?;

        let mut id_collection = Vec::<String>::new();

        // Gets first capture group where the mail id is stored
        for id in MAIL_ID_REGEX.captures_iter(&response_str).map( |capture| capture.get(1).unwrap()) {

            let id = id.as_str().to_string();

            id_collection.push(id);

        }

        Ok(id_collection)

    }

}

/// An `Iterator` whose purpose is to reduce resource consumption by only requesting message subject and content to the server when `next()` is called.
pub struct MailMessageIterator {
    message_ids:Vec<String>,
    temp_server_identifier:String,
}

impl Iterator for MailMessageIterator {

    type Item = MailMessage;

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(id) = self.message_ids.pop() {

            GmailnatorInbox::get_message_by_id(&self.temp_server_identifier, &id).ok()

        } else {
            None
        }

    }

}