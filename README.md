[![Generic badge](https://img.shields.io/crates/l/gmailnator)]() [![Generic badge](https://img.shields.io/crates/v/gmailnator)]()

# Goals

This library has been designed to be <b>ultra high level</b> and <b>simple</b>, see the examples below.

# Examples may not always be up to date, refer to the [docs.rs](https://docs.rs/gmailnator/*/gmailnator/) documentation.

#### The main object is the <i>GmailnatorInbox</i> object, to generate a new mailbox :

```rust
extern crate gmailnator;
use gmailnator::GmailnatorInbox;
...
let inbox = GmailnatorInbox::new().expect("Error occured when creating the inbox.");
```

#### Getting the current email address string associated to the GmailnatorInbox instance :

```rust
let address:&str = inbox.get_address();
```

#### Display potentially received messages :

```rust
let messages = inbox.get_messages().expect("Failed to retrieve messages.");

for message in messages {

    let title = message.get_subject();
    let body = message.get_raw_content();

    println!("Title : {}\nBody : {}", title, body);

}
```
