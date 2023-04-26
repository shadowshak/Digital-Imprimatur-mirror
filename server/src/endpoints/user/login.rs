use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{models::{Role, AccessToken, UserId}, controllers::{Controller, UserLoginError}};

#[derive(Deserialize, Serialize)]
pub struct UserLoginRequest {
    user_name:      String,
    password:       String,
}

#[derive(Deserialize, Serialize)]
pub struct UserLoginResponse {
    user_id:        UserId,
    token:          AccessToken,
    role:           Role,

}

pub async fn login(
    Json(UserLoginRequest {
        user_name,
        password,
    }): Json<UserLoginRequest>)
    -> Result<Json<UserLoginResponse>, StatusCode>
{
    let mut session = Controller::session().await;

    let (user_id, token, role)
        = match session.login(user_name, password).await
    {
        Ok(data) => data,
        // todo: handle error
        Err(UserLoginError::UsernameInvalid) => return Err(StatusCode::UNAUTHORIZED),
        Err(UserLoginError::PasswordInvalid) => return Err(StatusCode::FORBIDDEN),
        Err(UserLoginError::TwoUsersWithSameUsername) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        Err(UserLoginError::DatabaseError) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let response = UserLoginResponse {
        user_id,
        token,
        role
    };

    Ok(Json(response))
}