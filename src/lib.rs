#[macro_use]
extern crate lazy_static;

mod mail;
mod endpoint;
mod regexes;
mod httphelper;

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