use axum::http::StatusCode;
use chrono::Local;

use crate::{models::{AccessToken, SubId, SubmissionMetadata, UserId, SubmissionStatus, DocId}, endpoints::doc::DocumentMetadata};

use super::{Controller, session::UserVerifyError, data};

pub struct DocumentController {

}

pub enum SubmissionError {
    InvalidAccessToken,
    TokenTimedOut,
    InvalidPermissions,
    DatabaseError
}

pub enum DocumentError {
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
        let user_id = {
            let mut session = Controller::session().await;

            match session.verify_session(token, vec![ ]) {
                Ok(user_id) => user_id,

                Err(UserVerifyError::InvalidAccessToken) => return Err(SubmissionError::InvalidAccessToken),
                Err(UserVerifyError::TokenTimedOut) => return Err(SubmissionError::TokenTimedOut),
                Err(UserVerifyError::InvalidPermissions) => return Err(SubmissionError::InvalidPermissions),
            }
        };

        let sub_id = SubId::new();

        // dates
        let creation    = Local::now();
        let last_update = Local::now();

        let status = SubmissionStatus::AwaitingSubmission;

        // create the submission
        let mut database = Controller::database().await;

        match database.execute(
            r#"
                INSERT INTO Submissions (sub_id, name, author, description, creation, last_update, status)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#, &[ &sub_id, &name, &author, &description, &creation, &last_update, &status ]).await
        {
            Ok(1) => { },
            _ => return Err(SubmissionError::DatabaseError)
        }

        match database.execute(r#"
            INSERT INTO Permissions (user_id, sub_id, role)
            VALUES ($1, $2, $3)
        "#, &[ &user_id, &sub_id, &"admin" ]).await {
            Ok(1) => {}
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

        match database.execute(r#"
           DELETE FROM Permissions
           WHERE sub_id = $1
        "#, &[&sub_id]).await
        {
           Ok(_) => { }
           _ => return Err(SubmissionError::DatabaseError)
        };

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
                SELECT name, author, description, creation, last_update, status
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
        let status      = rows[0].get(5);

        Ok(SubmissionMetadata {
            name,
            author,
            description,
            creation,
            last_update,
            status,
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

    pub async fn create_document(
        &mut self,
        token: AccessToken,
        sub_id: SubId,
        document: Vec<u8>) -> Result<DocId, SubmissionError>
    {
        let user_id = {
            let mut session = Controller::session().await;

            match session.verify_session(token, vec![ ]) {
                Ok(user_id) => user_id,

                Err(UserVerifyError::InvalidAccessToken) => return Err(SubmissionError::InvalidAccessToken),
                Err(UserVerifyError::TokenTimedOut) => return Err(SubmissionError::TokenTimedOut),
                Err(UserVerifyError::InvalidPermissions) => return Err(SubmissionError::InvalidPermissions),
            }
        };

        let doc_id = DocId::new();

        let mut database = Controller::database().await;

        match database.execute(r#"
            INSERT INTO Documents (doc_id, sub_id, creator, content)
            VALUES ($1, $2, $3, $4);
        "#, &[ &doc_id, &sub_id, &user_id, &document  ]).await
        {
            Ok(1) => { },
            _ => return Err(SubmissionError::DatabaseError)
        }

        Ok(doc_id)
    }

    pub async fn delete_document(
        &mut self,
        token: AccessToken,
        doc_id: DocId) -> Result<(), SubmissionError>
    {
        {
            let mut session = Controller::session().await;

            match session.verify_session(token, vec![ ]) {
                Ok(_) => { },

                Err(UserVerifyError::InvalidAccessToken) => return Err(SubmissionError::InvalidAccessToken),
                Err(UserVerifyError::TokenTimedOut) => return Err(SubmissionError::TokenTimedOut),
                Err(UserVerifyError::InvalidPermissions) => return Err(SubmissionError::InvalidPermissions),
            }
        }

        let mut database = Controller::database().await;

        match database.execute(
            r#"
                DELETE FROM Documents
                WHERE doc_id = $1
            "#, &[ &doc_id ]).await
        {
            Ok(1) => Ok(()),
            _ => Err(SubmissionError::DatabaseError)
        }
    }

    pub async fn get_document_metadata(
        &mut self,
        token: AccessToken,
        doc_id: DocId) -> Result<DocumentMetadata, SubmissionError>
    {
        {
            let mut session = Controller::session().await;

            match session.verify_session(token, vec![ ]) {
                Ok(_) => { },

                Err(UserVerifyError::InvalidAccessToken) => return Err(SubmissionError::InvalidAccessToken),
                Err(UserVerifyError::TokenTimedOut) => return Err(SubmissionError::TokenTimedOut),
                Err(UserVerifyError::InvalidPermissions) => return Err(SubmissionError::InvalidPermissions),
            }
        }

        let mut database = Controller::database().await;

        let rows = match database.query(
            r#"
                SELECT creator, creation, last_update
                FROM Documents
                WHERE doc_id = $1
            "#, &[ &doc_id ]).await
        {
            Ok(rows) => rows,

            _ => return Err(SubmissionError::DatabaseError)
        };

        if rows.len() != 1 {
            return Err(SubmissionError::DatabaseError)
        }

        let creator     = rows[0].get(0);
        let creation    = rows[0].get(1);
        let last_update = rows[0].get(2);

        Ok(DocumentMetadata {
            creation,
            last_update,
            creator,
        })
    }

    pub async fn download_document(
        &mut self,
        token: AccessToken,
        doc_id: DocId) -> Result<Vec<u8>, SubmissionError>
    {
        {
            let mut session = Controller::session().await;

            match session.verify_session(token, vec![ ]) {
                Ok(_) => { },

                Err(UserVerifyError::InvalidAccessToken) => return Err(SubmissionError::InvalidAccessToken),
                Err(UserVerifyError::TokenTimedOut) => return Err(SubmissionError::TokenTimedOut),
                Err(UserVerifyError::InvalidPermissions) => return Err(SubmissionError::InvalidPermissions),
            }
        }

        let mut database = Controller::database().await;

        let rows = match database.query(
            r#"
                SELECT content
                FROM Documents
                WHERE doc_id = $1
            "#, &[ &doc_id ]).await
        {
            Ok(rows) => rows,

            _ => return Err(SubmissionError::DatabaseError)
        };

        if rows.len() != 1 {
            return Err(SubmissionError::DatabaseError);
        }

        let content = rows[0].get(0);

        Ok(content)
    }
}

impl Default for DocumentController {
    fn default() -> Self {
        Self {

        }
    }
}

impl SubmissionError {
    pub fn into_status_code(self) -> StatusCode {
        match self {
            SubmissionError::InvalidAccessToken => return StatusCode::FORBIDDEN,
            SubmissionError::InvalidPermissions => return StatusCode::UNAUTHORIZED,
            SubmissionError::TokenTimedOut => return StatusCode::FORBIDDEN,
            SubmissionError::DatabaseError => return StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}