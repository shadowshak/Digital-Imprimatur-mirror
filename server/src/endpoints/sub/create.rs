use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{models::{AccessToken, SubId}, endpoints::check_length, controllers::{Controller, SubmissionCreateError}};

#[derive(Deserialize, Serialize)]
pub struct SubCreateRequest {
    token:          AccessToken,
    name:           String,
    author:         String,
    description:    String,
}

#[derive(Deserialize, Serialize)]
pub struct SubCreateResponse {
    sub_id:         SubId,
}

const MAX_SUB_NAME_LENGTH: usize = 100;
const MAX_SUB_AUTHOR_LENGTH: usize = 100;
const MAX_SUB_DESC_LENGTH: usize = 1000;

pub async fn create(
    Json(SubCreateRequest {
        token,
        name,
        author,
        description,
    }): Json<SubCreateRequest>)
    -> Result<Json<SubCreateResponse>, StatusCode>
{
    check_length(&name, MAX_SUB_NAME_LENGTH)?;
    check_length(&author, MAX_SUB_AUTHOR_LENGTH)?;
    check_length(&description, MAX_SUB_DESC_LENGTH)?;

    let mut documents = Controller::document().await;

    let sub_id =
        match documents.create_submission(token, name, author, description).await {
            Ok(sub_id) => sub_id,

            Err(SubmissionCreateError::InvalidAccessToken) => return Err(StatusCode::FORBIDDEN),
            Err(SubmissionCreateError::InvalidPermissions) => return Err(StatusCode::UNAUTHORIZED),
            Err(SubmissionCreateError::TokenTimedOut) => return Err(StatusCode::FORBIDDEN),
            Err(SubmissionCreateError::DatabaseError) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

    let response = SubCreateResponse {
        sub_id,
    };

    Ok(Json(response))
}