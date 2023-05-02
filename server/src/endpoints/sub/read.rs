use axum::{http::StatusCode, Json};
use serde::{Serialize, Deserialize};

use crate::{models::{AccessToken, SubId, SubmissionMetadata}, controllers::{Controller, SubmissionError}};

#[derive(Serialize, Deserialize)]
pub struct SubReadRequest {
    token:          AccessToken,
    submission_id:  SubId,
}

#[derive(Serialize, Deserialize)]
pub struct SubReadResponse {
    metadata:       SubmissionMetadata
}

pub async fn read(
    Json(SubReadRequest {
        token,
        submission_id,
    }): Json<SubReadRequest>)
    -> Result<Json<SubReadResponse>, StatusCode>
{
    /*
        User has been assigned to the submission 
     */
    let mut documents = Controller::document().await;

    let metadata =
    match documents.get_submission_metadata(token, submission_id).await {
        Ok(metadata) => metadata,

        Err(SubmissionError::InvalidAccessToken) => return Err(StatusCode::FORBIDDEN),
        Err(SubmissionError::InvalidPermissions) => return Err(StatusCode::UNAUTHORIZED),
        Err(SubmissionError::TokenTimedOut) => return Err(StatusCode::FORBIDDEN),
        Err(SubmissionError::DatabaseError) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let response = SubReadResponse {
        metadata,
    };

    return Ok(Json(response))
}