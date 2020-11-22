use regex::Regex;

lazy_static! {

    pub static ref MAIL_ID_REGEX:Regex = Regex::new(r"messageid\\/#(.*?)\\").unwrap();

}