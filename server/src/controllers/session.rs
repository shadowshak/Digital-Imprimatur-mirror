use std::collections::HashMap;

use chrono::{Local, DateTime, Duration};

use crate::models::{AccessToken, UserId, Role};

use super::{Controller, UserLoginError};

pub struct Session {
    access_token:   AccessToken,
    user_id:        UserId,
    expires:        DateTime<Local>,

    // cached data
}

pub struct SessionController {
    sessions: HashMap<AccessToken, Session>
}

impl SessionController {
    ///
    /// Login a user, and return an access token
    /// 
    pub async fn login(
        &mut self,
        username: String,
        password: String,
        role:     Role) -> Result<(UserId, AccessToken), UserLoginError>
    {
        let mut user = Controller::user().await;

        let user_id = user.login_user(username, password, role).await?;

        // create an access token
        let access_token    = AccessToken::new();
        let expiry_time     = Duration::days(30);

        // create a session 
        let session = Session {
            access_token,
            user_id,
            expires: Local::now()
                .checked_add_signed(expiry_time)
                .expect("Congratulations, you broke the space-time continuum!"),
        };

        // track the session
        // and make sure we don't have any duplicates
        if self.sessions.contains_key(&access_token) {
            return Err(UserLoginError::DatabaseError)
        }
        self.sessions.insert(access_token, session);

        Ok((user_id, access_token))
    }

    ///
    /// Invalidates a login session
    /// 
    pub async fn invalidate(
        &mut self,
        user_id: UserId,
        access_token: AccessToken) -> Result<(), UserInvalidateError>
    {
        // check that the access token is valid
        let Some(session) = self.sessions.get(&access_token) else {
            return Err(UserInvalidateError::InvalidAccessToken);
        };

        if session.user_id != user_id {
            return Err(UserInvalidateError::InvalidUserId);
        }

        // Delete the session
        self.sessions.remove(&access_token);

        Ok(())
    }

    pub fn verify_session(
        &mut self,
        token:       AccessToken,
        permissions: Vec<String>) -> Result<(), UserVerifyError>
    {
        let Some(session) = self.sessions.get(&token) else {
            //return Err(UserLoginError::InvalidAccessToken);
            todo!()
        };

        // check that the session is still valid
        if session.expires < Local::now() {
            // Invalidate the session
            self.sessions.remove(&token);

            // return an error

            todo!()
        }

        // check that the user has the required permissions

        Ok(())
    }
}

impl Default for SessionController {
    fn default() -> Self {
        Self { sessions: Default::default() }
    }
}

pub enum UserVerifyError {
    InvalidAccessToken,
    TokenTimedOut,
    InvalidPermissions,
}

pub enum UserInvalidateError {
    InvalidAccessToken,
    InvalidUserId,
}