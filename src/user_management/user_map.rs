use lazy_static::lazy_static;
use std::collections::HashMap;
use uuid::{Uuid, uuid};

use crate::events::UserRole;

pub struct User {
    pub userid: Uuid,
    pub username: String,
    pub role: UserRole,
}

lazy_static! {
    pub static ref USERS_MAP: HashMap<Uuid, User> = {
        let mut m = HashMap::new();
        m.insert(
            uuid!("385ea9a6-d121-48a0-96ec-6ec3b353ca5b"),
            User {
                userid: uuid!("385ea9a6-d121-48a0-96ec-6ec3b353ca5b"),
                username: String::from("Alice"),
                role: UserRole::Admin,
            },
        );
        m.insert(
            uuid!("0ab85ffd-7e62-4cdf-82b0-eb359e50c7b9"),
            User {
                userid: uuid!("0ab85ffd-7e62-4cdf-82b0-eb359e50c7b9"),
                username: String::from("Tony"),
                role: UserRole::User,
            },
        );

        m
    };
}

pub fn get_users_map() -> &'static HashMap<Uuid, User> {
    &USERS_MAP
}
