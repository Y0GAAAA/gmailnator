//! [`GmailnatorInbox`]: struct.GmailnatorInbox.html
//! [`MailMessage`]: struct.MailMessage.html
//! [`MailMessageIterator`]: struct.MailMessageIterator.html
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
//! This creates `n` number of addresses, it must be used to create a large number of inboxes.
//! ```
//! use gmailnator::GmailnatorInbox;
//!
//! let n:u32 = 500;
//! let inboxes:Vec<GmailnatorInbox> = GmailnatorInbox::new_bulk(n).unwrap();
//! 
//! assert_eq!(inboxes.len() as u32, n); 
//! ```
//! 
//! Retrieve messages in a vector and display them via the container struct [`MailMessage`]:
//! ```
//! use gmailnator::{GmailnatorInbox, MailMessage};
//! # let inbox = GmailnatorInbox::new().unwrap();
//! let messages:Vec<MailMessage> = inbox.get_messages_iter().unwrap().collect();
//! 
//! for message in messages {
//! 
//!     let title = message.get_subject();
//! 
//!     let raw_body = message.get_raw_content();
//!     let decoded_body = message.decode_content().unwrap();
//! 
//!     // raw_body     = &lt;You&gt; Where did you put the &quot;thing&quot; ?
//!     // decoded_body = <You> Where did you put the "thing" ?
//! 
//!     println!("Title : {}\nBody : {}", title, decoded_body);
//! 
//! }
//! ```
//! 
//! To search for a particular message, use the [`MailMessageIterator`] :
//! ```
//! use gmailnator::{GmailnatorInbox, MailMessage, MailMessageIterator};
//! # let inbox = GmailnatorInbox::new().unwrap();
//! let mut messages_iter:MailMessageIterator = inbox.get_messages_iter().unwrap();
//!
//! let find_result = messages_iter.find(|m| m.get_subject() == "Confirm your order");
//! 
//! if let Some(confirmation_message) = find_result {
//! 
//!   //Confirm your order :)
//! 
//! }
//!  ```


#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

#[macro_use] extern crate lazy_static;

extern crate ureq;
extern crate regex;
extern crate futures;
extern crate url;
extern crate scraper;
extern crate htmlescape;
extern crate serde;
extern crate serde_json;

mod errors;
mod mail;
mod endpoint;
mod regexes;
mod http;

pub use mail::{MailMessage, GmailnatorInbox, MailMessageIterator, Error};
pub use errors::GmailnatorError;

#[cfg(test)]
mod passive_tests {

    extern crate serde;
    extern crate serde_json;

    use crate::mail::{GmailnatorInbox, MailMessage};

    #[test]
    fn create_inbox() {

        let inbox = GmailnatorInbox::new().expect("Failed to create an inbox."); 

        let address = inbox.get_address();

        println!("Inbox created with email : {}", address);

        assert!(address.contains('@'))

    }

    #[test]
    fn retrieve_messages_iter() {

      let inbox = GmailnatorInbox::new().unwrap();

      let mut message_iter = inbox.get_messages_iter().unwrap();

      assert!(message_iter.next().is_none());

    }

    #[test]
    fn create_inbox_from_existing_address() {

        let new_address = GmailnatorInbox::new().unwrap();
        let new_address = new_address.get_address();
        
        let inbox = GmailnatorInbox::from_address(new_address).unwrap();

        assert_eq!(inbox.get_address(), new_address);

    }  

    #[test]
    fn create_bulk() {
        
        let count:u32 = 1;

        let inboxes = GmailnatorInbox::new_bulk(count).unwrap();   

        assert_eq!(inboxes.len() as u32, count);

    }

    #[test]
    fn create_bulk_larger() {
        
        let count:u32 = 1000;

        let inboxes = GmailnatorInbox::new_bulk(count).unwrap();   

        assert_eq!(inboxes.len() as u32, count);

    }

    #[test]
    fn create_bulk_invalid() {

        assert!(GmailnatorInbox::new_bulk(0).is_err())

    }

    #[test]
    fn parse_mail_message_classic() {

        let json = "{\"subject\":\"<b>subject<\\/b><div>5 hrs ago<hr \\/><\\/div>\",\"content\":\"<div dir=\\\"ltr\\\">content<\\/div>\\r\\n\"}"; 

        let message = MailMessage::parse(json).unwrap();
 
        assert_eq!(message.get_subject(), "subject");

        assert_eq!(message.decode_content().unwrap(), "content");
        assert_eq!(message.get_raw_content(), "content");

    }

    #[test]
    fn serialize_mail_message() {

      let s = String::from("Subject !!!!:!!!!!!!!!!!");
      let c = String::from("CONFIRMATION_HTML_BODY");

      let message = MailMessage::new(s, c);

      let json_string = serde_json::to_string(&message).unwrap();

      let message_reconstructed:MailMessage = serde_json::from_str(&json_string).unwrap();

      assert_eq!(message_reconstructed.get_raw_content(), "CONFIRMATION_HTML_BODY");

    }

}