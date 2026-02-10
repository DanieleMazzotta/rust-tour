use chrono::DateTime;
use uuid::Uuid;

#[derive(PartialEq, Debug, Clone)]
pub enum UserRole {
    Admin,
    User,
}

#[derive(PartialEq, Debug)]
pub enum LogoutCause {
    UserInitiated,
    Forced,
    Inactivity,
}

#[derive(PartialEq, Debug)]
pub struct LoginEvent {
    pub userid: Uuid,
    pub username: String,
    pub role: UserRole,
    pub timestamp: DateTime<chrono::Utc>,
}

#[derive(PartialEq, Debug)]
pub struct LogoutEvent {
    pub userid: Uuid,
    pub username: String,
    pub role: UserRole,
    pub timestamp: DateTime<chrono::Utc>,
    pub cause: LogoutCause,
}
