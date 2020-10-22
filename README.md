# Goals

This library has been designed to be <b>ultra high level</b> and <b>simple</b> <i>(see the examples below)</i>. Performance has been taken in account but the host's slowness in a real bottleneck.

# Examples

### The main object is the <i>GmailnatorInbox</i> object, to generate a new mailbox :

```rust
extern crate gmailnator;
use gmailnator::mail::GmailnatorInbox;
...
let inbox = GmailnatorInbox::new().expect("Error occured when creating the inbox.");
```

### Getting the current email address string associated to the GmailnatorInbox instance :

```rust
let address = inbox.get_address(); returns an &str
```

### Display potentially received messages :

```rust
let messages = inbox.get_messages().expect("Failed to retrieve messages.");

for message in messages {

    let title = message.get_title();
    let body = message.get_raw_content();

    println!("Title : {}\nBody : {}", title, body);

}
```
