use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{models::{Role, UserId, AccessToken}, controllers::{Controller, UserGetInfoError}};

#[derive(Deserialize, Serialize)]
pub struct UserGetInfoRequest {
    token:          AccessToken,
    user:           UserId,
}

#[derive(Deserialize, Serialize)]
pub struct UserGetInfoResponse {
    user_name:      String,
    email:          String,
    first_name:     String,
    last_name:      String,

    role:           Role
}

pub async fn get_info(
    Json(UserGetInfoRequest {
        token,
        user: user_id,
    }): Json<UserGetInfoRequest>)
    -> Result<Json<UserGetInfoResponse>, StatusCode>
{
    let mut user = Controller::session().await;

    let user_info = match user.get_user_info(user_id, token).await {
        Ok(user_info) => user_info,
        Err(UserGetInfoError::UserIdInvalid) => return Err(StatusCode::NO_CONTENT),
        Err(UserGetInfoError::TokenInvalid) => return Err(StatusCode::UNAUTHORIZED),
        Err(UserGetInfoError::DatabaseError) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let response = UserGetInfoResponse {
        user_name:      user_info.username,
        email:          user_info.email,
        first_name:     user_info.first_name,
        last_name:      user_info.last_name,

        role:           user_info.role,
    };

    Ok(Json(response))
}