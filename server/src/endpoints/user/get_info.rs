use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{models::{Role, UserId, AccessToken}};

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
        user,
    }): Json<UserGetInfoRequest>)
    -> Result<Json<UserGetInfoResponse>, StatusCode>
{

    let response = UserGetInfoResponse {
        user_name:      "scotthahn1".into(),
        email:          "scotthahn@saintpaul.com".into(),
        first_name:     "Scott".into(),
        last_name:      "Hahn".into(),

        role:           Role::Publisher
    };

    Ok(Json(response))
}