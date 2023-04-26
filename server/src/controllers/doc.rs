use std::error::Error;

use chrono::Local;

use crate::models::{AccessToken, SubId};

use super::{Controller, session::UserVerifyError};

pub struct DocumentController {

}

pub enum SubmissionCreateError {
    InvalidAccessToken,
    TokenTimedOut,
    InvalidPermissions,
    DatabaseError
}

impl DocumentController {
    pub async fn create_submission(
        &mut self,
        token: AccessToken,
        name:  String,
        author: String,
        description: String) -> Result<SubId, SubmissionCreateError>
    {
        // check that the token is valid
        {
            let mut session = Controller::session().await;

            if let Err(e) = session.verify_session(token, vec![ ]) {
                match e {
                    UserVerifyError::InvalidAccessToken => return Err(SubmissionCreateError::InvalidAccessToken),
                    UserVerifyError::TokenTimedOut => return Err(SubmissionCreateError::TokenTimedOut),
                    UserVerifyError::InvalidPermissions => return Err(SubmissionCreateError::InvalidPermissions),
                }
            }
        }

        let sub_id = SubId::new();

        // dates
        let creation    = Local::now();
        let last_update = Local::now();

        // create the submission
        let mut database = Controller::database().await;

        match database.execute(
            r#"
                INSERT INTO Submissions (sub_id, name, author, description, creation, last_update)
                VALUES ($1, $2, $3, $4, $5, $6)
            "#, &[ &sub_id, &name, &author, &description, &creation, &last_update ]).await
        {
            Ok(1) => { },
            _ => return Err(SubmissionCreateError::DatabaseError)
        }

        Ok(sub_id)
    }
}

impl Default for DocumentController {
    fn default() -> Self {
        Self {

        }
    }
}