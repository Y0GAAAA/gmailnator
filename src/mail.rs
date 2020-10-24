extern crate scraper;

use crate::endpoint::*;
use crate::regexes::{CSRF_REGEX, MAIL_ID_REGEX};
use crate::http::{UrlQuery, get_error, EXPIRED_TOKEN};
use crate::errors::GmailnatorError;
use crate::token::{renew_token, get_token_sync};

use scraper::{Html, Selector};
 
/// Default error for the crate.
pub type Error = GmailnatorError;

/// A structure that contains an e-mail subject and it's raw content, which can be accessed by the following two functions :
/// ```
/// use gmailnator::MailMessage;
/// 
/// let message:MailMessage = MailMessage::empty();
/// 
/// let subject:&str = message.get_subject();
/// let content:&str = message.get_raw_content();
/// 
/// assert!(subject.is_empty());
/// assert!(content.is_empty());
/// ```
#[derive(Debug)]
pub struct MailMessage {
    subject:String,
    content:String,
}

lazy_static! {

    static ref SUBJECT_SELECTOR:Selector = Selector::parse("b").unwrap();
    static ref BODY_SELECTOR:Selector = Selector::parse("div:nth-child(4)").unwrap();

}

impl MailMessage {

    pub(crate) fn from(subject:String, content:String) -> Self {
        Self { subject, content }
    }

    pub(crate) fn parse(response_fragment:&str) -> Result<Self, ()> {

        let fragment = Html::parse_fragment(response_fragment);

        let mut subject_container = fragment.select(&SUBJECT_SELECTOR);
        let mut body_container = fragment.select(&BODY_SELECTOR);

        let subject_item = subject_container.next();
        let body_item = body_container.next();

        if subject_item.is_none() || body_item.is_none() {
            return Err(());
        }

        let subject = subject_item.unwrap().inner_html();
        let body = body_item.unwrap().inner_html();

        Ok(MailMessage::from(subject, body))
        
    }

    /// Returns an empty mail message.
    pub fn empty() -> Self {
        Self {
            subject:String::default(),
            content:String::default(),
        }
    }

    /// Gets the message's subject.
    pub fn get_subject(&self) -> &str {
        &self.subject
    }

    /// Gets the message's raw content.
    pub fn get_raw_content(&self) -> &str {
        &self.content
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

        renew_token(false)?;
        
        let mut email_request = get_request_from_endpoint(GmailnatorEndpoint::GetEmail);
        let mut mail_query = GmailnatorInbox::get_tokened_query();
        
        mail_query.add("action", "GenerateEmail");
        mail_query.add("data%5B%5D", "2");
 
        let email_response = email_request.send_string(&mail_query.to_query_string());

        if let Some(error_code) = get_error(&email_response) {
            if error_code == EXPIRED_TOKEN {
            
                renew_token(true)?;
            
                return GmailnatorInbox::new();

            } else {

                return Err(GmailnatorError::ServerError(error_code));

            }

        }

        let response_str = email_response.into_string().unwrap();

        let server_id = GmailnatorInbox::get_temp_server_id(&response_str)?;

        return Ok(
            Self {
                mail_address:response_str,
                temp_server:server_id,
            }
        );

    }

    /// Creates a new inbox from an  already existing gmailnator address. 
    /// Warning : an invalid address will not return an Error.
    /// ```
    /// # use gmailnator::GmailnatorInbox;
    /// let valid   = GmailnatorInbox::from_address("deedtmp+[...]@gmail.com").unwrap();
    /// let invalid = GmailnatorInbox::from_address("invalid.email@gmail.com").unwrap();
    /// ```
    pub fn from_address(address:&str) -> Result<Self, Error> {

        renew_token(false)?;

        let temp_server_id = GmailnatorInbox::get_temp_server_id(address)?;

        Ok(Self {
            mail_address:address.to_string(),
            temp_server:temp_server_id,
        })

    }

    /// Returns the received e-mail(s).
    pub fn get_messages(&self) -> Result<Vec<MailMessage>, Error> {

        let mut inbox_request = get_request_from_endpoint(GmailnatorEndpoint::GetInbox);

        let mut query = GmailnatorInbox::get_tokened_query();
        
        query.add("action", "LoadMailList");
        query.add("Email_address", &self.mail_address);

        let inbox_response = inbox_request.send_string(&query.to_query_string());

        if let Some(error_code) = get_error(&inbox_response) {

            if error_code == EXPIRED_TOKEN {

                renew_token(true)?;

                return self.get_messages();

            } else {

                return Err(GmailnatorError::ServerError(error_code));

            }
        
        }

        let response_str = inbox_response.into_string().unwrap();

        let mut inbox_messages:Vec<MailMessage> = Vec::new();

        for id in MAIL_ID_REGEX.captures_iter(&response_str)
                                    .map(|capture| capture.get(1).unwrap()) {

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

    pub(crate)fn get_new_token() -> Result<String, Error> {

        let mut main_page_request = get_request_from_endpoint(GmailnatorEndpoint::GetToken);

        let response = main_page_request.call();

        if response.error() { return Err(GmailnatorError::ServerError(response.status())); }

        let set_cookie_value = response.header("Set-Cookie").unwrap();

        if let Some(match_groups) = CSRF_REGEX.captures(set_cookie_value) {
            if let Some(final_token) = match_groups.get(1) {
                
                return Ok(final_token.as_str().to_string());

            }
        }

        Err(GmailnatorError::TokenParsingError(set_cookie_value.to_string()))

    }

    fn get_message_by_id(&self, message_id:&str) -> Result<MailMessage, ()> {

        let mut get_message_request = get_request_from_endpoint(GmailnatorEndpoint::GetMessage);

        let mut get_message_query = GmailnatorInbox::get_tokened_query();

        get_message_query.add("action", "get_message");
        get_message_query.add("message_id", message_id);
        get_message_query.add("email", &self.temp_server);

        let message_response = get_message_request.send_string(&get_message_query.to_query_string());

        if message_response.error() { return Err(()); }

        let parsable_message = message_response.into_string().unwrap();

        MailMessage::parse(&parsable_message)

    }

    fn get_tokened_query() -> UrlQuery {

        let csrf_value = get_token_sync(); 

        let mut tokened_query = UrlQuery::new();

        if csrf_value.is_none() {
            return tokened_query;   
        }

        tokened_query.add("csrf_gmailnator_token", &csrf_value.unwrap());

        tokened_query

    }

    fn get_temp_server_id(mail_address:&str) -> Result<String, Error> {

        let server_id:Vec<&str> = mail_address.split('+').collect();

        if server_id.is_empty() {
            return Err(GmailnatorError::MailServerParsingError(mail_address.to_string()));
        }

        let server_id = server_id[0].to_string();

        Ok(server_id)

    }

} 