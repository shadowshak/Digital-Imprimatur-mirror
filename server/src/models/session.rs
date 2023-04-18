use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use uuid::Uuid;

uuid_based! (AccessToken);
uuid_based! (InvalidToken);
uuid_based! (UserId);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, ToSql, FromSql)]
pub enum Role {
    Admin,
    Publisher,
    User,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id:        UserId,
    pub access_token:   AccessToken,
    pub expiration:     DateTime<Local>,
    // capabilities
    pub role:           Role,
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            Role::Admin => serializer.serialize_str("admin"),
            Role::Publisher => serializer.serialize_str("publisher"),
            Role::User => serializer.serialize_str("user"),
        }
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let str_role = String::deserialize(deserializer)?;

        let role =
        match str_role.as_str() {
            "admin" => Role::Admin,
            "publisher" => Role::Publisher,
            "user" => Role::User,
            _ => {
                return Err(serde::de::Error::custom(
                    "invalid role",
                ));
            }
        };

        Ok(role)
    }
}