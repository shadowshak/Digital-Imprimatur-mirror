use std::collections::HashMap;

use chrono::{Local, DateTime, Duration};

use crate::models::{AccessToken, UserId, Role, UserInfo};

use super::{Controller, UserLoginError, UserGetInfoError};

pub struct Session {
    access_token:   AccessToken,
    user_id:        UserId,
    expires:        DateTime<Local>,

    // cached data
}

pub struct SessionController {
    sessions:   HashMap<AccessToken, Session>,
    info_cache: HashMap<UserId, UserInfo>,
}

impl SessionController {
    ///
    /// Login a user, and return an access token
    /// 
    pub async fn login(
        &mut self,
        username: String,
        password: String) -> Result<(UserId, AccessToken, Role), UserLoginError>
    {
        let mut user = Controller::user().await;

        let (user_id, role) = user.login_user(username, password).await?;

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

        Ok((user_id, access_token, role))
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

    pub async fn get_user_info(
        &mut self,
        user: UserId,
        token: AccessToken) -> Result<UserInfo, UserGetInfoError>
    {
        // Verify token
        if let Err(_) = self.verify_session(token, vec![]) {
            return Err(UserGetInfoError::TokenInvalid);
        }

        if let Some(user_info) = self.info_cache.get(&user) {
            return Ok(user_info.clone());
        }

        let mut user_controller = Controller::user().await;
        return user_controller.get_info(user).await;
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
        Self {
            sessions: Default::default(),
            info_cache: Default::default(),
        }
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