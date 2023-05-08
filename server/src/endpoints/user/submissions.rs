use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{models::{Role, UserId, AccessToken, SubmissionMetadata, SubId}, controllers::{Controller, SubmissionError}};

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
    let submissions = {
        let mut session = Controller::session().await;

        match session.get_submissions_by_user(token).await
        {
            Ok(submissions) => submissions,

            Err(SubmissionError::InvalidAccessToken) => return Err(StatusCode::FORBIDDEN),
            Err(SubmissionError::InvalidPermissions) => return Err(StatusCode::UNAUTHORIZED),
            Err(SubmissionError::TokenTimedOut) => return Err(StatusCode::FORBIDDEN),
            Err(SubmissionError::DatabaseError) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    };

    println!("{submissions:?}");

    let mut documents = Controller::document().await;

    let mut submission_details = vec![];

    for id in submissions {
        let meta = match documents.get_submission_metadata(token, id).await {
            Ok(meta) => meta,
            Err(e) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        };

        println!("{meta:?}");

        submission_details.push(UserSubmission { id, meta });
    }

    let response = UserSubmissionsResponse {
        submissions: submission_details,
    };

    Ok(Json(response))
}