extern crate scraper;

use crate::endpoint;
use crate::regexes;
use crate::httphelper;

use scraper::{Html, Selector};

use httphelper::UrlQuery;

use endpoint::*;
use regexes::*;

pub struct MailMessage {
    object:String,
    content:String,
}

lazy_static! {

    static ref TITLE_SELECTOR:Selector = Selector::parse("b").unwrap();
    static ref BODY_SELECTOR:Selector = Selector::parse("div:nth-child(4)").unwrap();

}

impl MailMessage {

    pub fn from(object:String, content:String) -> Self {
        Self { object, content }
    }

    pub fn parse(response_fragment:&str) -> Result<Self, ()> {

        let fragment = Html::parse_fragment(response_fragment);

        let mut title_container = fragment.select(&TITLE_SELECTOR);
        let mut body_container = fragment.select(&BODY_SELECTOR);

        let title_item = title_container.next();
        let body_item = body_container.next();

        if title_item.is_none() || body_item.is_none() {
            return Err(());
        }

        let title = title_item.unwrap().inner_html();
        let body = body_item.unwrap().inner_html();

        Ok(MailMessage::from(title, body))
        
    }

    pub fn get_title(&self) -> &str {
        &self.object
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

}

#[derive(Debug)]
pub struct GmailnatorInbox {

    mail_address:String,    //COMPLETE E-MAIL | Ex : extmp+blabla@gmail.com
    temp_server:String,     //SERVER ID       | Ex : extmp

    csrf_token:String,      //Cross site request forgery token | Ex : 0732953026e73c6631577d6b5d019788

}

impl GmailnatorInbox {

    pub fn new() -> Result<Self, ()> {

        let token = GmailnatorInbox::get_new_token()?;

        let mut email_request = get_request_from_endpoint(
                                GmailnatorEndpoint::GetEmail, 
                                Some(&token)
                            );

        let mut mail_query = UrlQuery::new(); 
        
        mail_query.add("csrf_gmailnator_token", &token);
        mail_query.add("action", "GenerateEmail");

        mail_query.add("data%5B%5D", "2");

        let email_response = email_request.send_string(&mail_query.to_query_string());

        if email_response.error() {
            return Err(());
        }

        let response_str = email_response.into_string().unwrap();

        let server_id:Vec<&str> = response_str.split('+').collect();
        let server_id = server_id[0].to_string();

        Ok(
            Self {
                mail_address:response_str,
                temp_server:server_id,
                csrf_token:token
            }
        )

    }

    fn get_new_token() -> Result<String, ()> {

        let mut main_page_request = get_request_from_endpoint(GmailnatorEndpoint::GetToken, None);

        let response = main_page_request.call();

        if response.error() { return Err(()); }

        let set_cookie_value = response.header("Set-Cookie").unwrap();

        if let Some(match_groups) = CSRF_REGEX.captures(set_cookie_value) {
            if let Some(final_token) = match_groups.get(1) {
                
                return Ok(final_token.as_str().to_string());

            }
        }

        Err(())

    }

    pub fn get_messages(&self) -> Result<Vec<MailMessage>, ()> {

        let mut inbox_request = get_request_from_endpoint(
                                    GmailnatorEndpoint::GetInbox, 
                                    Some(&self.csrf_token)
                                );

        let mut query = self.get_tokened_query();
        
        query.add("action", "LoadMailList");
        query.add("Email_address", &self.mail_address);

        let mut inbox_messages:Vec<MailMessage> = Vec::new();

        let inbox_response = inbox_request.send_string(&query.to_query_string());

        if inbox_response.error() { return Err(()); }

        let response_str = inbox_response.into_string().unwrap();

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

    fn get_message_by_id(&self, message_id:&str) -> Result<MailMessage, ()> {

        let mut get_message_request = get_request_from_endpoint(GmailnatorEndpoint::GetMessage, Some(&self.csrf_token));

        let mut get_message_query = self.get_tokened_query();

        get_message_query.add("action", "get_message");
        get_message_query.add("message_id", message_id);
        get_message_query.add("email", &self.temp_server);

        let message_response = get_message_request.send_string(&get_message_query.to_query_string());

        if message_response.error() { return Err(()); }

        let parsable_message = message_response.into_string().unwrap();

        MailMessage::parse(&parsable_message)

    }

    pub fn get_address(&self) -> &str {
        &self.mail_address
    }

    fn get_tokened_query(&self) -> UrlQuery {

        let mut tokened_query = UrlQuery::new();

        tokened_query.add("csrf_gmailnator_token", &self.csrf_token);

        tokened_query

    }

} 