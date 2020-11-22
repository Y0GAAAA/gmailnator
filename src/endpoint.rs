use ureq::Request;

impl GmailnatorEndpoint { //IMPLEMENT EP

    pub fn to_request(self) -> GmailnatorRequest {

        match self {
            
            GmailnatorEndpoint::GetEmail        => GmailnatorRequest::from(HttpMethod::Post, "https://gmailnator.com/index/indexquery"),
            GmailnatorEndpoint::GetEmailBulk    => GmailnatorRequest::from(HttpMethod::Post, "https://gmailnator.com/bulk-emails"),

            GmailnatorEndpoint::GetInbox        => GmailnatorRequest::from(HttpMethod::Post, "https://gmailnator.com/mailbox/mailboxquery"),
            GmailnatorEndpoint::GetMessage      => GmailnatorRequest::from(HttpMethod::Post, "https://gmailnator.com/mailbox/get_single_message"),
        
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

    GetEmail,
    GetInbox,
    GetMessage,
    GetEmailBulk,

}

const EMPTY_CSRF_COOKIE:&'static str = "csrf_gmailnator_cookie=;";
const DEFAULT_USER_AGENT:&'static str = "Mozilla/5.0 (Windows NT 6.4; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/41.0.2225.0 Safari/537.36";

const URL_ENCODED_CONTENT_TYPE:&'static str = "application/x-www-form-urlencoded; charset=UTF-8";

pub fn get_request_from_endpoint(ep:GmailnatorEndpoint) -> Request {

    let request_param = ep.to_request();

    let (url, method) = (request_param.get_url(), request_param.get_method());

    let mut base_req = match method {

        HttpMethod::Get  => ureq::get(url),
        HttpMethod::Post => ureq::post(url),

    };

    base_req.set("Cookie", EMPTY_CSRF_COOKIE);
    base_req.set("User-Agent", DEFAULT_USER_AGENT);

    if method == HttpMethod::Post {
        base_req.set("Content-Type", URL_ENCODED_CONTENT_TYPE); 
    }

    base_req

}