use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::models::{AccessToken, UserId};

#[derive(Deserialize, Serialize)]
pub struct UserInvalidateRequest {
    user_id:        UserId,
    token:          AccessToken,
}

#[derive(Deserialize, Serialize)]
pub struct UserInvalidateResponse;

pub async fn invalidate(
    Json(UserInvalidateRequest {
        user_id,
        token,
    }): Json<UserInvalidateRequest>)
    -> Result<Json<UserInvalidateResponse>, StatusCode>
{

    let response = UserInvalidateResponse;

    Ok(Json(response))
}