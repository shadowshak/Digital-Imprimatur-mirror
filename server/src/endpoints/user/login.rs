use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::models::{Role, AccessToken, UserId};

#[derive(Deserialize, Serialize)]
pub struct UserLoginRequest {
    user_id:        UserId,
    password:       String,
    role:           Role,
}

#[derive(Deserialize, Serialize)]
pub struct UserLoginResponse {
    token:          AccessToken,
}

pub async fn login(
    Json(UserLoginRequest {
        user_id,
        password,
        role,
    }): Json<UserLoginRequest>)
    -> Result<Json<UserLoginResponse>, StatusCode>
{

    let response = UserLoginResponse {
        token: AccessToken::new(),
    };

    Ok(Json(response))
}