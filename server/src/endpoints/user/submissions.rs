use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{models::{Role, UserId, AccessToken, SubmissionMetadata, SubId}};

#[derive(Deserialize, Serialize)]
pub struct UserSubmissionsRequest {
    token:          AccessToken,
}

#[derive(Deserialize, Serialize)]
pub struct UserSubmissionsResponse {
    submissions: Vec<UserSubmission>
}

#[derive(Deserialize, Serialize)]
pub struct UserSubmission {
    id:     SubId,
    meta:   SubmissionMetadata,
}

pub async fn submissions(
    Json(UserSubmissionsRequest {
        token,
    }): Json<UserSubmissionsRequest>)
    -> Result<Json<UserSubmissionsResponse>, StatusCode>
{

    let response = UserSubmissionsResponse {
        submissions: Vec::new()
    };

    Ok(Json(response))
}