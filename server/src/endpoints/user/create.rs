use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{models::{Role, UserId}, endpoints::check_length, controllers::{Controller, UserCreateError}};

#[derive(Deserialize, Serialize)]
pub struct UserCreateRequest {
    user_name:      String,
    email:          String,
    first_name:     String,
    last_name:      String,
    password:       String,

    role:           Role,
}

#[derive(Deserialize, Serialize)]
pub struct UserCreateResponse {
    user_id:        UserId,
}

const MAX_USER_NAME_LENGTH: usize = 32;
const MAX_EMAIL_LENGTH: usize = 64;
const MAX_FIRST_NAME_LENGTH: usize = 32;
const MAX_LAST_NAME_LENGTH: usize = 32;
const MAX_PASSWORD_LENGTH: usize = 32;

pub async fn create(
    Json(UserCreateRequest {
        user_name,
        email,
        first_name,
        last_name,
        password,
        role,
    }): Json<UserCreateRequest>)
    -> Result<Json<UserCreateResponse>, StatusCode>
{
    check_length(&user_name, MAX_USER_NAME_LENGTH)?;
    check_length(&email, MAX_EMAIL_LENGTH)?;
    check_length(&first_name, MAX_FIRST_NAME_LENGTH)?;
    check_length(&last_name, MAX_LAST_NAME_LENGTH)?;
    check_length(&password, MAX_PASSWORD_LENGTH)?;

    let mut user = Controller::user().await;

    let user_id = match user.create_user(user_name, email, first_name, last_name, password, role).await {
        Ok(user_id) => user_id,

        Err(UserCreateError::UsernameTaken) => return Err(StatusCode::UNAUTHORIZED),
        Err(UserCreateError::EmailTaken) => return Err(StatusCode::CONFLICT),
        Err(UserCreateError::PasswordInvalid) => return Err(StatusCode::FORBIDDEN),
        Err(UserCreateError::DatabaseError) => return Err(StatusCode::INTERNAL_SERVER_ERROR),

        _ => todo!()
    };

    let user_id = UserId::new();

    let response = UserCreateResponse {
        user_id,
    };

    Ok(Json(response))
}