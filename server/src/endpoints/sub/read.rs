use axum::{http::StatusCode, Json};
use serde::{Serialize, Deserialize};

use crate::models::{AccessToken, SubId, SubmissionMetadata};

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

    let metadata = SubmissionMetadata {
        name: "The Lamb's Supper".into(),
        author: "Scott Hahn".into(),
        description: "A book about the Eucharist.".into()
    };

    let response = SubReadResponse {
        metadata,
    };

    return Ok(Json(response))
}