extern crate regex;

use regex::Regex;

lazy_static! {

    pub static ref CSRF_REGEX:Regex = Regex::new("csrf_gmailnator_cookie=(.*?);").unwrap();
    pub static ref MAIL_ID_REGEX:Regex = Regex::new(r"messageid\\/#(.*?)\\").unwrap();

}