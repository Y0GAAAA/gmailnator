use url::form_urlencoded::byte_serialize;
use ureq::{Response, Request};
use crate::mail::Error;

/// Defines the maximum retry count if an http response indicates an `internal server error (500)`
const QUERY_MAX_TRY:u32 = 2;

/// The internal server error http error code sent back by the server.
const INTERNAL_SERVER_ERROR:u16 = 500;

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

    let mut last_error:Error = Error::ServerError(0);

    let payload = query.to_query_string();

    for _ in 0..QUERY_MAX_TRY {

        let response = request.send_string(&payload);

        if let Some(error_code) = get_error(&response) { //If error gets returned

            last_error = Error::ServerError(error_code); //Set last error whatever it is

            if error_code == INTERNAL_SERVER_ERROR { //If it is an internal server error we iterate one more time if 'available'

                continue;

            } else { //Else we return directly the error because it's most likely not gonna get fixed by re-requesting

                return Err(last_error)

            }

        } else { //Return the response content if request succeeded

            return Ok(response.into_string().unwrap_or_default());

        }

    }

    Err(last_error) //If multiples tries weren't enough, desesperately return the error :^(
    
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