mod events;
mod user_management;

use crate::{
    events::LogoutCause,
    user_management::login_service::{handle_login, handle_logout},
};
use std::{
    io::{self, Error},
    thread::{self},
    time::Duration,
};
use uuid::uuid;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Starting...");

    let userid = uuid!("385ea9a6-d121-48a0-96ec-6ec3b353ca5b");
    let password = String::from("123");

    let result = handle_login(userid, password);
    match result {
        Ok(event) => {
            println!("Login successful: {event:?}");
        }
        Err(e) => {
            return Err(Error::other(e));
        }
    }

    thread::sleep(Duration::from_millis(700));

    let logout_result = handle_logout(userid, LogoutCause::UserInitiated);
    match logout_result {
        Ok(event) => {
            println!("Logout successful: {event:?}");
        }
        Err(e) => {
            return Err(Error::other(e));
        }
    }

    Ok(())
}
