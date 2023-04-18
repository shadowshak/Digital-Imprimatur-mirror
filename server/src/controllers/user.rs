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
        let mut database = Controller::database();

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

    pub async fn login_user(
        &mut self,
        username:   String,
        password:   String,
        role:       Role) -> Result<UserId, UserLoginError>
    {
        let mut database = Controller::database();

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

    // the function needs to take the user id, the old password, and the new password
    // the function needs to check that the old password is correct
    // the function needs to check that the new password is valid
    // the function needs to hash the new password
    // the function needs to update the database with the new password
    // the function needs to return an error if the user id is invalid
    // the function needs to return an error if the old password is invalid
    fn change_password(&mut self, user_id: UserId, old_password: String, new_password: String) -> Result<(), ()> {
        let mut database = Controller::database();


        return Ok(())
    }

    // get user details
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