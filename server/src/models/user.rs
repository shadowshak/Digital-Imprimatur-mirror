use super::Role;

#[derive(Clone)]
pub struct UserInfo {
    pub username:   String,
    pub email:      String,
    pub first_name: String,
    pub last_name:  String,

    pub role:       Role
}