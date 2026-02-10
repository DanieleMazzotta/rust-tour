#[cfg(test)]
use pretty_assertions::assert_eq;

use lazy_static::lazy_static;
use uuid::Uuid;

use soc_gs::user_management::{login_service::handle_login, user_map::USERS_MAP};

#[test]
fn fail_on_empty_password() {
    let result = handle_login(Uuid::new_v4(), String::from(""));
    let result2 = handle_login(Uuid::new_v4(), String::new());

    assert_eq!(true, result.is_err());
    assert_eq!(true, result2.is_err());
}

#[test]
fn fail_on_user_not_found() {
    USERS_MAP.lock().unwrap().clear();

    let result = handle_login(Uuid::new_v4(), String::from("1234"));

    assert_eq!(true, result.is_err());
}
