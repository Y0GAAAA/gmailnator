use lazy_static;
use std::sync::Mutex;

use crate::mail::{GmailnatorInbox, Error};

lazy_static!{

    pub static ref CSRF_TOKEN:Mutex<Option<String>> = Mutex::new(None);

}

/// Returns the current token value, returning a String instead of an 
/// Option<String> shouldn't be a problem since we don't access 


pub fn set_token_sync(token:String) {

    let mut token_ref = CSRF_TOKEN.lock().unwrap();

    *token_ref = Some(token);

}

pub fn renew_token(overwrite:bool) -> Result<(), Error> {

    if CSRF_TOKEN.lock().unwrap().is_some() && !overwrite {
        return Ok(());
    }

    let token_result = GmailnatorInbox::get_new_token();

    if let Ok(token) = token_result {

        set_token_sync(token);

        Ok(())

    } else {

        Err(token_result.unwrap_err())

    }

}