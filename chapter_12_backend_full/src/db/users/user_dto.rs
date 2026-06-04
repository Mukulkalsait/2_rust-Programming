use chrono::{DateTime, Utc};
use core::str;
use serde::{Deserialize, Serialize};

use crate::db::{User, UserRole};

#[derive(validator::Validate, Debug, Default, Serialize, Deserialize)]
/// # TO complete registration we will be needing things explicitley.
/// > =>
///     - Valid Name
///     - Valid Email
///     - Valid Password
///     - Same Password Repeted
/// * validator::Validator provide us the pre-build **Procedural Macro** to validate this.
pub struct RegistorUserDataTransObj {
    #[validate(length(min = 3, message = "Name is required."))]
    pub name: String,

    #[validate(length(min = 1, message = "Email is required."), email(message = "Email is inValid."))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be 6+ letters to be strong."))]
    pub password: String,

    #[validate(
        length(min = 1, message = "conform Password is required."),
        must_match(other = "password", message = "Password must match.")
    )]
    #[serde(rename = "passwordConfirm")]
    pub password_confirm: String,
}

#[derive(validator::Validate, serde::Serialize, Deserialize)]
/// # To complete Login we will be needing some explicit Informaiotn.
/// > =>
///     - Valid Email
///     - Valid Password
/// * validator::Validator provide pre-build **Procedural Macro** to validate this.
pub struct LoginUserDataTransObj {
    #[validate(length(min = 1, message = "Email is require."), email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be atleast 8 charecters."))]
    pub password: String,
}

#[derive(validator::Validate, serde::Serialize, Deserialize, Debug)]
/// # To Find/Filter User we will be needing some explicit Informaiotn.
/// > =>
///     - list of informaiotn
/// * validator::Validator provide pre-build **Procedural Macro** to validate this.
pub struct FilterUserDataTransObj {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub verified: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl FilterUserDataTransObj {
    /// Dont know what exactly we are doing here? are we chagintin tiypes fo the user input fields?
    pub fn filter_user(user: &User) -> Self {
        FilterUserDataTransObj {
            id: user.id.to_string(),
            name: user.name.to_string(),
            email: user.email.to_string(),
            role: user.role.to_string().into(),
            verified: user.verified,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }

    /// what we did with fn filter_user SINGLE we are iteratoring all user and puting here.
    pub fn filter_users(user: &[User]) -> Vec<FilterUserDataTransObj> { user.iter().map(FilterUserDataTransObj::filter_user).collect() }
}

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDataTransObj,
}

#[derive(Debug, serde::Deserialize, Serialize)]
pub struct UserResponseDataTransObj {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct UserListResponseDataTransObj {
    pub status: String,
    pub users: Vec<FilterUserDataTransObj>,
    pub resualt: i64,
}

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct UserLoginResponseDataTransObj {
    pub status: String,
    pub token: String,
}

#[derive(Debug, serde::Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Debug, serde::Serialize, Deserialize, validator::Validate)]
pub struct NameUpdateDataTransObj {
    #[validate(length(min = 1, message = "Name is Required"))]
    pub name: String,
}

fn validate_user_role(role: &UserRole) -> Result<(), validator::ValidationError> {
    match role {
        UserRole::Admin | UserRole::User => Ok(()),
        // _ => Err(validator::ValidationError::new("Invalid Role")),
        // R: no need of 👆 since we coverd all cases
    }
}

#[derive(Debug, serde::Deserialize, Serialize, validator::Validate)]
pub struct RoleUpdateDataTransObj {
    #[validate(custom(function = validate_user_role))]
    pub role: UserRole,
}

#[derive(Debug, serde::Deserialize, Serialize, validator::Validate, Default)]
pub struct UserPasswordUpdateDataTransObj {
    #[validate(length(min = 8, message = "Password must be 8 letters long."))]
    pub new_password: String,
    #[validate(
        length(min = 8, message = "Password must be 8 letters long."),
        must_match(other = "new_password", message = "New Passwords are not matching.")
    )]
    pub new_password_confirm: String,

    #[validate(length(min = 8, message = "Valid new password is required."))]
    pub old_password: String,
}

#[derive(Debug, serde::Deserialize, Serialize, validator::Validate)]
pub struct VerifyEmailQueryDataTransObj {
    #[validate(length(min = 1, message = "Token is required."))]
    pub token: String,
}
#[derive(Debug, serde::Serialize, Deserialize, validator::Validate)]
pub struct ForagePasswordRequestDataTransObj {
    #[validate(length(min = 6, message = "Email is required."), email(message = "Valid Email Is Required."))]
    pub email: String,
}

#[derive(Debug, serde::Deserialize, Serialize, validator::Validate)]
pub struct ResetPasswordRequestDataTransObj {
    #[validate(length(min = 1, message = "Token is required."))]
    pub token: String,
    #[validate(length(min = 8, message = "Valid New Password is required."))]
    pub new_password: String,
    #[validate(
        length(min = 8, message = "Valid New Password is required."),
        must_match(other = "new_password", message = "Both Password must match.")
    )]
    pub new_password_confirm: String,
}
