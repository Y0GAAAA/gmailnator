extern crate scraper;
extern crate htmlescape;

use crate::endpoint::*;
use crate::regexes::MAIL_ID_REGEX;
use crate::http::{UrlQuery, get_response_content};
use crate::errors::GmailnatorError;

use scraper::{Html, Selector};
use htmlescape::decode_html; 

const MIN_BULK_COUNT:u32 = 1;
const MAX_BULK_COUNT:u32 = 1000;

const BODY_SPLITTER:&'static str = "<hr />"; 

/// Default error for the crate.
pub type Error = GmailnatorError;

/// A structure that contains an e-mail subject and its content under two forms : raw, and html decoded.
#[derive(Debug)]
pub struct MailMessage {
    subject:String,
    raw_content:String,
}

lazy_static! {

    static ref SUBJECT_SELECTOR:Selector = Selector::parse("b").unwrap();
    static ref BODY_SELECTOR:Selector = Selector::parse("div:nth-child(4)").unwrap();

    static ref BULK_EMAIL_SELECTOR:Selector = Selector::parse("#email-list-message > a").unwrap();

}

impl MailMessage {

    pub(crate) fn parse(response_fragment:&str) -> Result<Self, Error> {

        let fragment = Html::parse_fragment(response_fragment);

        let mut subject_container = fragment.select(&SUBJECT_SELECTOR);
        let mut body_container = fragment.select(&BODY_SELECTOR);

        let subject_item = subject_container.next();
        let body_item = body_container.next();

        if subject_item.is_none() {
            return Err(Error::HtmlParsingError(response_fragment.to_string()));
        }

        let is_html_content = body_item.is_none();

        let subject = subject_item.unwrap().inner_html();
        let subject = decode_html(&subject).unwrap_or_default();

        let raw_body = match is_html_content {

            false => body_item.unwrap().inner_html(),
            true => {
            
                if let Some(mut start_index) = response_fragment.find(BODY_SPLITTER) {

                    start_index += BODY_SPLITTER.len();

                    response_fragment[start_index..].to_string()

                } else {

                    String::default()

                }
            
            }

        };
        
        Ok(Self {
            subject,
            raw_content:raw_body,
        })
        
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
#[derive(Debug)]
pub struct GmailnatorInbox {

    mail_address:String,    //COMPLETE E-MAIL | Ex : extmp+blabla@gmail.com
    temp_server:String,     //SERVER ID       | Ex : extmp

}

impl GmailnatorInbox {

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

        if count < MIN_BULK_COUNT || count > MAX_BULK_COUNT {
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

    /// Returns the received e-mail(s).
    pub fn get_messages(&self) -> Result<Vec<MailMessage>, Error> {

        let inbox_request = get_request_from_endpoint(GmailnatorEndpoint::GetInbox);

        let mut query = GmailnatorInbox::get_tokened_query();
        
        query.add("action", "LoadMailList");
        query.add("Email_address", &self.mail_address);

        let response_str = get_response_content(inbox_request, query)?;

        let mut inbox_messages:Vec<MailMessage> = Vec::new();

        // Gets first capture group where the mail id is stored
        for id in MAIL_ID_REGEX.captures_iter(&response_str).map( |capture| capture.get(1).unwrap()) {

            let final_id = id.as_str().to_string();

            let message = self.get_message_by_id(&final_id);

            if let Ok(legit_message) = message {

                inbox_messages.push(legit_message);

            }

        }
        
        Ok(inbox_messages)

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

    fn get_message_by_id(&self, message_id:&str) -> Result<MailMessage, Error> {

        let get_message_request = get_request_from_endpoint(GmailnatorEndpoint::GetMessage);

        let mut get_message_query = GmailnatorInbox::get_tokened_query();

        get_message_query.add("action", "get_message");
        get_message_query.add("message_id", message_id);
        get_message_query.add("email", &self.temp_server);
        
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

} 