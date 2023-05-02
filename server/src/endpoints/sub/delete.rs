use axum::{http::StatusCode, Json};
use serde::{Serialize, Deserialize};

use crate::{models::{AccessToken, SubId}, controllers::{Controller, SubmissionDeleteError}};


#[derive(Serialize, Deserialize)]
pub struct SubDeleteRequest {
    token:          AccessToken,
    submission_id:  SubId,
}

#[derive(Serialize, Deserialize)]
pub struct SubDeleteResponse;

pub async fn delete(
    Json(SubDeleteRequest {
        token,
        submission_id,
    }): Json<SubDeleteRequest>)
    -> Result<Json<SubDeleteResponse>, StatusCode>
{
    /*
        User must be a publisher 
        User must be associated with the submission User must have delete permissions 
    */
    let mut documents = Controller::document().await;

    match documents.delete_submission(token, submission_id).await {
        Ok(_) => { },

        Err(SubmissionDeleteError::InvalidAccessToken) => return Err(StatusCode::FORBIDDEN),
        Err(SubmissionDeleteError::InvalidPermissions) => return Err(StatusCode::UNAUTHORIZED),
        Err(SubmissionDeleteError::TokenTimedOut) => return Err(StatusCode::FORBIDDEN),
        Err(SubmissionDeleteError::DatabaseError) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    let response = SubDeleteResponse;

    return Ok(Json(response))
}