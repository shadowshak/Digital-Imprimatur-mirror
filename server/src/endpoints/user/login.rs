use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{models::{Role, AccessToken, UserId}, controllers::{Controller, UserLoginError}};

#[derive(Deserialize, Serialize)]
pub struct UserLoginRequest {
    user_name:      String,
    password:       String,
    role:           Role,
}

#[derive(Deserialize, Serialize)]
pub struct UserLoginResponse {
    user_id:        UserId,
    token:          AccessToken,

}

pub async fn login(
    Json(UserLoginRequest {
        user_name,
        password,
        role,
    }): Json<UserLoginRequest>)
    -> Result<Json<UserLoginResponse>, StatusCode>
{
    let mut session = Controller::session().await;

    let (user_id, token)
        = match session.login(user_name, password, role).await
    {
        Ok(data) => data,
        // todo: handle error
        Err(UserLoginError::UsernameInvalid) => return Err(StatusCode::UNAUTHORIZED),
        Err(UserLoginError::PasswordInvalid) => return Err(StatusCode::FORBIDDEN),
        Err(UserLoginError::InvalidRole) => return Err(StatusCode::NOT_ACCEPTABLE),
        Err(UserLoginError::TwoUsersWithSameUsername) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        Err(UserLoginError::DatabaseError) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let response = UserLoginResponse {
        user_id,
        token,
    };

    Ok(Json(response))
}