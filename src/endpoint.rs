extern crate ureq;

use ureq::Request;

use crate::token::CSRF_TOKEN;

impl GmailnatorEndpoint { //IMPLEMENT EP

    pub fn to_request(self) -> GmailnatorRequest {

        match self {

            GmailnatorEndpoint::GetToken    => GmailnatorRequest::from(HttpMethod::Get, "https://gmailnator.com/"),
            
            GmailnatorEndpoint::GetEmail    => GmailnatorRequest::from(HttpMethod::Post, "https://gmailnator.com/index/indexquery"),

            GmailnatorEndpoint::GetInbox    => GmailnatorRequest::from(HttpMethod::Post, "https://gmailnator.com/mailbox/mailboxquery"),
            GmailnatorEndpoint::GetMessage  => GmailnatorRequest::from(HttpMethod::Post, "https://gmailnator.com/mailbox/get_single_message/"),
        
        }

    }

}

pub struct GmailnatorRequest {

    method:HttpMethod,
    url:String,

}

impl GmailnatorRequest {

    pub fn from(method:HttpMethod, url:&str) -> Self {
        Self {method, url:url.to_string()}
    }

    pub fn get_method(&self) -> HttpMethod {
        self.method
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

}

#[derive(Copy, Clone, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
}

#[derive(Copy, Clone, PartialEq)]
pub enum GmailnatorEndpoint {

    GetToken,
    GetEmail,
    GetInbox,
    GetMessage,

}

pub fn get_request_from_endpoint(ep:GmailnatorEndpoint) -> Request {

    let request_param = ep.to_request();

    let (url, method) = (request_param.get_url(), request_param.get_method());

    let mut base_req = match method {

        HttpMethod::Get  => ureq::get(url),
        HttpMethod::Post => ureq::post(url),

    };

    let guard = CSRF_TOKEN.lock().unwrap();
    let guard_value = guard.as_ref();

    if ep != GmailnatorEndpoint::GetToken && guard_value.is_some() {

        let auth_cookie = format!("csrf_gmailnator_cookie={}", guard_value.unwrap());

        base_req.set("Cookie", &auth_cookie);

    }

    if method == HttpMethod::Post {
        base_req.set("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8"); 
    }

    base_req.set("User-Agent", "Mozilla/5.0 (Windows NT 6.4; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2225.0 Safari/537.36");

    base_req

}