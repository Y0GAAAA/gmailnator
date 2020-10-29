extern crate url;
extern crate ureq;

use url::form_urlencoded::byte_serialize;
use ureq::{Response, Request};
use crate::mail::Error;

pub fn url_encode(s:&str) -> String {

    let encoded:String = byte_serialize(s.as_bytes()).collect();

    encoded

}

pub fn get_error(server_response:&Response) -> Option<u16> {

    if server_response.error() {

        Some(server_response.status())

    } else {

        None

    }

}

pub fn get_response_content(mut request:Request, query:UrlQuery) -> Result<String, Error> {

    let payload = &query.to_query_string();

    let response = request.send_string(payload);

    if let Some(error_code) = get_error(&response) {
        Err(Error::ServerError(error_code))
    } else {
        Ok(response.into_string().unwrap_or_default())
    }

}

pub struct UrlQuery {

    query_string:String,

}

impl UrlQuery {

    pub fn new() -> Self {
        Self {
            query_string:String::default()
        }
    }

    pub fn add(&mut self, key:&str, value:&str) {

        let is_first_entry = self.query_string.is_empty();

        let encoded_value = url_encode(value);

        if !is_first_entry {
            self.query_string.push('&');
        }

        self.query_string.push_str(key);
        self.query_string.push('=');
        self.query_string.push_str(&encoded_value);

    }

    pub fn to_query_string(self) -> String {
        self.query_string
    }

}