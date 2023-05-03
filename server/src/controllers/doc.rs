use chrono::Local;

use crate::models::{AccessToken, SubId, SubmissionMetadata, UserId};

use super::{Controller, session::UserVerifyError};

pub struct DocumentController {

}

pub enum SubmissionError {
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
        description: String) -> Result<SubId, SubmissionError>
    {
        // check that the token is valid
        {
            let mut session = Controller::session().await;

            if let Err(e) = session.verify_session(token, vec![ ]) {
                match e {
                    UserVerifyError::InvalidAccessToken => return Err(SubmissionError::InvalidAccessToken),
                    UserVerifyError::TokenTimedOut => return Err(SubmissionError::TokenTimedOut),
                    UserVerifyError::InvalidPermissions => return Err(SubmissionError::InvalidPermissions),
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
            _ => return Err(SubmissionError::DatabaseError)
        }

        Ok(sub_id)
    }

    pub async fn delete_submission(
        &mut self,
        token:  AccessToken,
        sub_id: SubId) -> Result<(), SubmissionError>
    {
        // check that the token is valid
        {
            let mut session = Controller::session().await;

            // todo: add the correct permissions
            if let Err(e) = session.verify_session(token, vec![ ]) {
                match e {
                    UserVerifyError::InvalidAccessToken => return Err(SubmissionError::InvalidAccessToken),
                    UserVerifyError::TokenTimedOut => return Err(SubmissionError::TokenTimedOut),
                    UserVerifyError::InvalidPermissions => return Err(SubmissionError::InvalidPermissions),
                }
            }
        }

         // delete the submission
         let mut database = Controller::database().await;

        match database.execute(
            r#"
                DELETE FROM Submissions
                WHERE sub_id = $1
            "#, &[&sub_id]).await
        {
            Ok(1) => Ok(()),

            _ => Err(SubmissionError::DatabaseError)
        }
    }

    pub async fn get_submission_metadata(
        &mut self,
        token:  AccessToken,
        sub_id: SubId) -> Result<SubmissionMetadata, SubmissionError>
    {
        // check that the token is valid
        {
            let mut session = Controller::session().await;

            // todo: add the correct permissions
            if let Err(e) = session.verify_session(token, vec![ ]) {
                match e {
                    UserVerifyError::InvalidAccessToken => return Err(SubmissionError::InvalidAccessToken),
                    UserVerifyError::TokenTimedOut => return Err(SubmissionError::TokenTimedOut),
                    UserVerifyError::InvalidPermissions => return Err(SubmissionError::InvalidPermissions),
                }
            }
        }

        let mut database = Controller::database().await;

        let rows = match database.query(
            r#"
                SELECT name, author, description, creation, last_update
                FROM Submissions
                WHERE sub_id = $1
            "#, &[&sub_id]).await
        {
            Ok(rows) => rows,

            _ => return Err(SubmissionError::DatabaseError)
        };

        if rows.len() != 1 {
            return Err(SubmissionError::DatabaseError)
        }

        let name        = rows[0].get(0);
        let author      = rows[0].get(1);
        let description = rows[0].get(2);
        let creation    = rows[0].get(3);
        let last_update = rows[0].get(4);

        Ok(SubmissionMetadata {
            name,
            author,
            description,
            creation,
            last_update
        })

    }

    pub async fn update_submission_metadata(
        &mut self,
        token: AccessToken,
        sub_id: SubId,
        metadata: SubmissionMetadata) -> Result<(), SubmissionError>
    {
        // check that the token is valid
        {
            let mut session = Controller::session().await;

            // todo: add the correct permissions
            if let Err(e) = session.verify_session(token, vec![ ]) {
                match e {
                    UserVerifyError::InvalidAccessToken => return Err(SubmissionError::InvalidAccessToken),
                    UserVerifyError::TokenTimedOut => return Err(SubmissionError::TokenTimedOut),
                    UserVerifyError::InvalidPermissions => return Err(SubmissionError::InvalidPermissions),
                }
            }
        }

        let mut database = Controller::database().await;

        match database.execute(
            r#"
                UPDATE Submissions
                SET name = $2, author = $3, description = $4, creation = $5, last_update = $6
                WHERE sub_id = $1
            "#, &[&sub_id, &metadata.name, &metadata.author, &metadata.description, &metadata.creation, &metadata.last_update]).await
        {
            Ok(1) => Ok(()),

            _ => return Err(SubmissionError::DatabaseError)
        }
    }
}

impl Default for DocumentController {
    fn default() -> Self {
        Self {

        }
    }
}