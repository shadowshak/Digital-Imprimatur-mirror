use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{models::{AccessToken, UserId}, controllers::{Controller, session::UserInvalidateError}};

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
    let mut session = Controller::session().await;

    if let Err(e) = session.invalidate(user_id, token).await {
        match e {
            UserInvalidateError::InvalidAccessToken => return Err(StatusCode::UNAUTHORIZED),
            UserInvalidateError::InvalidUserId => return Err(StatusCode::FORBIDDEN),
        }
    }

    let response = UserInvalidateResponse;

    Ok(Json(response))
}