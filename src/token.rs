use lazy_static;
use std::sync::Mutex;

use crate::mail::{GmailnatorInbox, Error};

lazy_static!{

    pub static ref CSRF_TOKEN:Mutex<Option<String>> = Mutex::new(None);

}

pub fn get_token_sync() -> Option<String> {

    let guard = CSRF_TOKEN.lock().unwrap();

    if guard.is_some() {

        return Some(guard.as_ref().unwrap().to_string());

    }

    None

}

pub fn set_token_sync(token:String) {

    let mut token_ref = CSRF_TOKEN.lock().unwrap();

    *token_ref = Some(token);

}

pub fn renew_token(overwrite:bool) -> Result<(), Error> {

    if get_token_sync().is_some() && !overwrite {
        return Ok(());
    }

    let token = GmailnatorInbox::get_new_token()?;

    set_token_sync(token);

    Ok(())

}