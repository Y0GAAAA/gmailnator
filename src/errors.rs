use std::error::Error;
use std::fmt;

/// Default error type for the crate.
#[derive(Debug)]
pub enum GmailnatorError {

    /// Stores the error code returned by the server.
    ServerError(u16),

    /// Stores the invalid Set-Cookie string sent by the server.
    TokenParsingError(String),

    /// Stores the invalid gmail address returned by the server.
    MailServerParsingError(String),
    
}

impl Error for GmailnatorError {}

impl fmt::Display for GmailnatorError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    
        let error_message = match &self {
            GmailnatorError::MailServerParsingError(address) => format!("Could not parse mail server of : {}", address),
            GmailnatorError::TokenParsingError(set_token) => format!("Could not parse the Set-Cookie header in : {}", set_token),
            GmailnatorError::ServerError(error_code) => format!("Server error-ed with status code : {}", error_code),
        };

        write!(f, "{}", error_message);
    
        Ok(())

    }

}