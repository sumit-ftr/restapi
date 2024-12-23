use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub _id: ObjectId,
    pub legal_name: String,
    pub email: String,
    pub dob: DateTime,
    pub password: String,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub country: Option<String>,
    // status: UserStatus,
    // pub created: DateTime,
    // pub last_login: DateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserStatus {
    Normal,
    Locked,
    Blocked,
    Deactivated,
}

#[derive(Deserialize, Debug)]
pub struct UnregisteredEntry {
    pub name: String,
    // pub email: String,
    pub dob: DateTime,
    pub otp: u32,
    pub password: Option<String>,
    pub register_status: RegisterStatus,
}

#[derive(Deserialize, PartialEq, Debug)]
pub enum RegisterStatus {
    Created,
    EmailVerified,
    PasswordSet,
}
