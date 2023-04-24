use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{models::{AccessToken}};

#[derive(Deserialize, Serialize)]
pub struct UserChangePasswordRequest {
    token:              AccessToken,
    current_password:   String,
    new_password:       String
}

#[derive(Deserialize, Serialize)]
pub struct UserChangePasswordResponse;

pub async fn change_password(
    Json(UserChangePasswordRequest {
        token,
        current_password,
        new_password,
    }): Json<UserChangePasswordRequest>)
    -> Result<Json<UserChangePasswordResponse>, StatusCode>
{

    let response = UserChangePasswordResponse;

    Ok(Json(response))
}