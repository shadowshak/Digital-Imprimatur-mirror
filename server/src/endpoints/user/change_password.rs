use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{models::{AccessToken}, controllers::{Controller, UserChangePasswordError}};

#[derive(Deserialize, Serialize)]
pub struct UserChangePasswordRequest {
    user_name:          String,
    current_password:   String,
    new_password:       String
}

#[derive(Deserialize, Serialize)]
pub struct UserChangePasswordResponse;

pub async fn change_password(
    Json(UserChangePasswordRequest {
        user_name,
        current_password,
        new_password,
    }): Json<UserChangePasswordRequest>)
    -> Result<Json<UserChangePasswordResponse>, StatusCode>
{
    let mut user = Controller::user().await;

    if let Err(e) = user.change_password(user_name, current_password, new_password).await {
        use UserChangePasswordError::*;

        match e {
            UsernameInvalid => return Err(StatusCode::FORBIDDEN),
            PasswordInvalid => return Err(StatusCode::UNAUTHORIZED),
            NewPasswordInvalid => return Err(StatusCode::EXPECTATION_FAILED),
            DatabaseError => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    let response = UserChangePasswordResponse;

    Ok(Json(response))
}