use axum::{http::StatusCode, Json};
use serde::{Serialize, Deserialize};

use crate::{models::{AccessToken, SubId, SubmissionMetadata}, controllers::{SubmissionError, Controller}};

#[derive(Serialize, Deserialize)]
pub struct SubUpdateRequest {
    token:          AccessToken,
    submission_id:  SubId,
    delta:          SubmissionMetadataDelta,
}

#[derive(Serialize, Deserialize)]
pub struct SubUpdateResponse;

#[derive(Serialize, Deserialize)]
pub struct SubmissionMetadataDelta {
    name:           Option<String>,
    author:         Option<String>,
    description:    Option<String>,
}

pub async fn update(
    Json(SubUpdateRequest {
        token,
        submission_id,
        delta,
    }): Json<SubUpdateRequest>)
    -> Result<Json<SubUpdateResponse>, StatusCode>
{
    /*
        User must be a publisher 
        User must be associated with the submission 
        User must have write permissions 
    */
    let mut documents = Controller::document().await;

    let mut metadata =
    match documents.get_submission_metadata(token, submission_id).await {
        Ok(metadata) => metadata,

        Err(SubmissionError::InvalidAccessToken) => return Err(StatusCode::FORBIDDEN),
        Err(SubmissionError::InvalidPermissions) => return Err(StatusCode::UNAUTHORIZED),
        Err(SubmissionError::TokenTimedOut) => return Err(StatusCode::FORBIDDEN),
        Err(SubmissionError::DatabaseError) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    metadata.name        = delta.name.unwrap_or(metadata.name);
    metadata.author      = delta.author.unwrap_or(metadata.author);
    metadata.description = delta.description.unwrap_or(metadata.description);

    if let Err(e) = documents.update_submission_metadata(token, submission_id, metadata).await {
        match e {
            SubmissionError::InvalidAccessToken => return Err(StatusCode::FORBIDDEN),
            SubmissionError::InvalidPermissions => return Err(StatusCode::UNAUTHORIZED),
            SubmissionError::TokenTimedOut => return Err(StatusCode::FORBIDDEN),
            SubmissionError::DatabaseError => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    let response = SubUpdateResponse;

    return Ok(Json(response))
}