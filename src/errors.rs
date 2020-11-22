use std::error::Error;
use std::fmt;

/// Default error enum for the crate, containing all the potential errors.
#[derive(Debug)]
pub enum GmailnatorError {

    /// Stores the error code returned by the server.
    ServerError(u16),

    /// Stores the invalid gmail address returned by the server.
    MailServerParsingError(String),
    
    /// Stores the unparsable html returned by the server.
    HtmlParsingError(String),

    /// Gets returned uniquely by the GmailnatorInbox::new_bulk(count:u32) method if the count argument has an invalid value. 
    InvalidCountError(u32),

    /// Gets returned when malformed html entities are found in an html document.
    HtmlDecodingError,

    /// Stores the invalid Json returned by the server.
    JsonParsingError(String),

}

impl Error for GmailnatorError {}

impl fmt::Display for GmailnatorError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
        let error_message = match &self {
            GmailnatorError::MailServerParsingError(address) => format!("Could not parse mail server of : {}", address),
            GmailnatorError::ServerError(error_code) => format!("Server error-ed with status code : {}", error_code),
            GmailnatorError::HtmlParsingError(html) => format!("Error occured while parsing the following html element : {}", html),
            GmailnatorError::InvalidCountError(count_value) => format!("Count argument has an invalid value ({})", count_value),
            GmailnatorError::HtmlDecodingError => "Malformed html entities were encountered.".to_string(),
            GmailnatorError::JsonParsingError(json) => format!("Invalid json string : {}", json),
        };

        write!(f, "{}", error_message)
    
    }

}