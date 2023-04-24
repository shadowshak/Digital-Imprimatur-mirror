use axum::{http::StatusCode, Json};
use serde::{Serialize, Deserialize};

use crate::models::{AccessToken, SubId, SubmissionMetadata};

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

    let mut metadata = SubmissionMetadata {
        name: "The Lamb's Supper".into(),
        author: "Scott Hahn".into(),
        description: "A book about the Eucharist.".into()
    };

    metadata.name        = delta.name.unwrap_or(metadata.name);
    metadata.author      = delta.author.unwrap_or(metadata.author);
    metadata.description = delta.description.unwrap_or(metadata.description);


    let response = SubUpdateResponse;

    return Ok(Json(response))
}