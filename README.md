[![Generic badge](https://img.shields.io/crates/l/gmailnator)]() [![Generic badge](https://img.shields.io/crates/v/gmailnator)]()

# Goals

This library has been designed to be <b>ultra high level</b> and <b>simple</b>, see the examples below.

# Examples

#### The main object is the <i>GmailnatorInbox</i> object, to generate a new mailbox :

```rust
extern crate gmailnator;
use gmailnator::GmailnatorInbox;
...
let inbox = GmailnatorInbox::new().expect("Error occured when creating the inbox.");
```

#### Getting the current email address string associated to the GmailnatorInbox instance :

```rust
let address = inbox.get_address(); returns an &str
```

#### Display potentially received messages :

```rust
let messages = inbox.get_messages().expect("Failed to retrieve messages.");

for message in messages {

    let title = message.get_title();
    let body = message.get_raw_content();

    println!("Title : {}\nBody : {}", title, body);

}
```
