//! [`GmailnatorInbox`]: struct.GmailnatorInbox.html
//! [`MailMessage`]: struct.MailMessage.html
//! This library contains objects to create a gmailnator inbox and read the messages it contains.
//! # Getting started : 
//! The main struct is the [`GmailnatorInbox`] struct, one instance contains one inbox associated to an email address.
//! 
//! This creates a new temporary gmail address :
//! ```
//! use gmailnator::GmailnatorInbox;
//! 
//! let inbox = GmailnatorInbox::new().unwrap();
//! ```
//! 
//! To get the associated mail address :
//! ```
//! # use gmailnator::GmailnatorInbox;
//! # let inbox = GmailnatorInbox::new().unwrap();
//! let address:&str = inbox.get_address();
//! ```
//! 
//! To retrieve messages and display them via the container struct [`MailMessage`]: 
//! ```
//! use gmailnator::{GmailnatorInbox, MailMessage};
//! # let inbox = GmailnatorInbox::new().unwrap();
//! let messages:Vec<MailMessage> = inbox.get_messages().expect("Failed to retrieve messages.");
//! 
//! for message in messages {
//! 
//!     let title = message.get_subject();
//!     let body = message.get_raw_content();
//! 
//!     println!("Title : {}\nBody : {}", title, body);
//! 
//! }
//! ```

#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

#[macro_use]
extern crate lazy_static;

mod errors;
mod mail;
mod endpoint;
mod regexes;
mod httphelper;

pub use mail::{MailMessage, GmailnatorInbox, Error};
pub use errors::GmailnatorError;

#[cfg(test)]
mod tests {

    use crate::mail::GmailnatorInbox;

    #[test]
    fn create_inbox() {

        let inbox = GmailnatorInbox::new().expect("Failed to create an inbox."); 

        let address = inbox.get_address();

        println!("Inbox created with email : {}", address);

        assert!(address.contains('@'))

    }

    #[test]
    fn retrieve_messages() {
        
        let inbox = GmailnatorInbox::new().expect("Failed to create an inbox."); 

        let messages = inbox.get_messages().expect("Failed to retrieve messages.");

        assert_eq!(messages.len(), 0);

    }

}