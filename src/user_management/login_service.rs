use uuid::Uuid;

use crate::{
    events::{LoginEvent, LogoutCause, LogoutEvent},
    user_management::user_map::get_users_map,
};

pub fn handle_login(userid: Uuid, password: String) -> Result<LoginEvent, String> {
    if password.is_empty() {
        return Err(String::from("Login forbidden"));
    }

    let user = get_users_map()
        .get(&userid)
        .ok_or(String::from("User not found"))?;

    let event = LoginEvent {
        userid,
        username: user.username.clone(),
        role: user.role.clone(),
        timestamp: chrono::offset::Utc::now(),
    };

    Ok(event)
}

pub fn handle_logout(userid: Uuid, cause: LogoutCause) -> Result<LogoutEvent, String> {
    if userid.is_nil() {
        return Err(String::from("Logout forbidden"));
    }

    let user = get_users_map()
        .get(&userid)
        .ok_or(String::from("User not found"))?;

    let event = LogoutEvent {
        userid,
        username: user.username.clone(),
        role: user.role.clone(),
        timestamp: chrono::offset::Utc::now(),
        cause,
    };

    Ok(event)
}
