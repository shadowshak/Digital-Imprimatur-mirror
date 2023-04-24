use crate::{models::{UserId, Role}, controllers::{Controller, data}};

pub struct UserController { }

impl UserController {
    ///
    /// Creates a new user
    /// 
    pub async fn create_user(
        &mut self,
        username:   String,
        email:      String,
        first_name: String,
        last_name:  String,
        password:   String,
        role:       Role) -> Result<UserId, UserCreateError>
    {
        let mut database = Controller::database().await;

        // Check that the username is not taken
        // Check that the email is not taken?
        let Ok(users_with_same_username) = database.query(
            r#"
                SELECT user_id FROM users
                 WHERE username = $1
            "#,
            &[&username]).await else {
            return Err(UserCreateError::DatabaseError);
        };

        if users_with_same_username.len() > 0 {
            // We already have a user with that name
            return Err(UserCreateError::UsernameTaken);
        }
 
        let Ok(users_with_same_email) = database.query(
            r#"
                SELECT email from users
                 WHERE email = $1
            "#,
            &[&username]).await else {
            return Err(UserCreateError::DatabaseError)
        };

        if users_with_same_email.len() > 0 {
            // We already have a user with that email
            return Err(UserCreateError::EmailTaken);
        }

        // Generate a user id
        let user_id = UserId::new();


        // Check that the password is valid

        

        // Hash the password
        let Ok(hashed_password) = bcrypt::hash(password, bcrypt::DEFAULT_COST) else {
            return Err(UserCreateError::PasswordInvalid);
        };

        // Add the user to the database
        let Ok(number_of_rows_affected) =
        database.excute(r#"
            INSERT INTO users (user_id, username, email, first_name, last_name, password, role)
                 VALUES ($1, $2, $3, $4, $5, $6, $7);
        "#, &[
            &user_id,
            &username,
            &email,
            &first_name,
            &last_name,
            &hashed_password,
            &role ]).await else {
            return Err(UserCreateError::DatabaseError);
        };

        if number_of_rows_affected != 1 {
            return Err(UserCreateError::DatabaseError);
        }

        return Ok(user_id)
    }

    ///
    /// Logs a user in
    /// Given the 
    /// - username
    /// - password
    /// - role
    /// 
    /// Returns the user id
    /// 
    /// Nothing is done to mark that the user is logged in
    /// 
    pub async fn login_user(
        &mut self,
        username:   String,
        password:   String,
        role:       Role) -> Result<UserId, UserLoginError>
    {
        let mut database = Controller::database().await;

        // Get the password, and user id from the database
        let Ok(rows) = database.query(
            r#"
                SELECT user_id, password, role FROM users
                 WHERE username = $1
            "#,
            &[&username]).await else {
            return Err(UserLoginError::DatabaseError);
        };

        let (user_id, password_hashed, actual_role) =
        match rows.len() {
            1 => {
                let user_id: UserId         = rows[0].get(0);
                let password_hashed: String = rows[0].get(1);
                let actual_role: Role       = rows[0].get(2);

                (user_id, password_hashed, actual_role)
            },
            0 => return Err(UserLoginError::UsernameInvalid),
            _ => return Err(UserLoginError::TwoUsersWithSameUsername)
        };

        // Check that the password is valid
        let Ok(true) = bcrypt::verify(password, &password_hashed) else {
            return Err(UserLoginError::PasswordInvalid);
        };

        if actual_role != role {
            return Err(UserLoginError::InvalidRole);
        }

        return Ok(user_id)
    }

   
    ///
    /// Updates the password for a user with the given user id. The old password must be correct. 
    /// 
    pub async fn change_password(
        &mut self,
        username:     String,
        old_password: String,
        new_password: String) -> Result<(), UserChangePasswordError>
    {
        let mut database = Controller::database().await;

        // Get the password, and user id from the database
        let Ok(rows) = database.query(
            r#"
                SELECT user_id, password FROM users
                 WHERE username = $1
            "#,
            &[&username]).await else {
            return Err(UserChangePasswordError::DatabaseError);
        };

        if rows.len() != 1 {
            return Err(UserChangePasswordError::UsernameInvalid);
        }

        let user_id: String         = rows[0].get(0);
        let password_hashed: String = rows[0].get(1);

        // Check that the password is valid
        let Ok(true) = bcrypt::verify(old_password, &password_hashed) else {
            return Err(UserChangePasswordError::PasswordInvalid);
        };

        // Check that the password is valid

        

        // Hash the password
        let Ok(hashed_password) = bcrypt::hash(new_password, bcrypt::DEFAULT_COST) else {
            return Err(UserChangePasswordError::NewPasswordInvalid);
        };

        // Update the database
        // and throw an error if one row was not updated
        let Ok(1) = database.excute(r#"
            UPDATE users
            SET password = $2
            WHERE username = $1
        "#, &[ &username, &hashed_password ]).await else {
            return Err(UserChangePasswordError::DatabaseError);
        };

        return Ok(())
    }

    // get user details
    pub async fn get_info(
        &mut self,
        user_id: UserId) -> Result<(), UserGetInfoError>
    {
        let mut database = Controller::database().await;

        let Ok(rows) = database.query(r#"
            SELECT username, email, first_name, last_name, role
              FROM users
             WHERE user_id = $1
        "#, &[ &user_id ]).await else {
            return Err(UserGetInfoError::DatabaseError);
        };

        if rows.len() != 1 {
            return Err(UserGetInfoError::UserIdInvalid);
        }

        let username:   String = rows[0].get(0);
        let email:      String = rows[0].get(1);
        let first_name: String = rows[0].get(2);
        let last_name:  String = rows[0].get(3);
        let role:       Role   = rows[0].get(4);



        return Ok(())
    }
}

pub enum UserCreateError {
    UsernameTaken,
    EmailTaken,
    PasswordInvalid,
    DatabaseError,
}

pub enum UserLoginError {
    UsernameInvalid,
    TwoUsersWithSameUsername,
    PasswordInvalid,
    InvalidRole,
    DatabaseError,
}

pub enum UserChangePasswordError {
    UsernameInvalid,
    PasswordInvalid,
    NewPasswordInvalid,
    DatabaseError,
}

pub enum UserGetInfoError {
    DatabaseError,
    UserIdInvalid,
}

impl Default for UserController {
    fn default() -> Self {
        Self {  }
    }
}