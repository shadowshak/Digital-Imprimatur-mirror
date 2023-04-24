use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use uuid::Uuid;

uuid_based! (AccessToken);
uuid_based! (InvalidToken);
uuid_based! (UserId);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

impl ToSql for Role {
    fn to_sql(&self, ty: &postgres_types::Type, out: &mut postgres_types::private::BytesMut) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
        where Self: Sized
    {
        match self {
            Role::Admin => "admin".to_sql(ty, out),
            Role::Publisher => "publisher".to_sql(ty, out),
            Role::User => "user".to_sql(ty, out),
        }
    }

    fn accepts(ty: &postgres_types::Type) -> bool
        where Self: Sized
    {
        <String as ToSql>::accepts(ty)
    }

    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
        match self {
            Role::Admin => "admin".to_sql_checked(ty, out),
            Role::Publisher => "publisher".to_sql_checked(ty, out),
            Role::User => "user".to_sql_checked(ty, out),
        }
    }
}

impl FromSql<'_> for Role {
    fn from_sql(ty: &postgres_types::Type, raw: &[u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let str_role = String::from_sql(ty, raw)?;

        let role =
        match str_role.as_str() {
            "admin" => Role::Admin,
            "publisher" => Role::Publisher,
            "user" => Role::User,
            _ => {
                // todo: just return an error
                panic!("corrupt table");
            }
        };

        Ok(role)
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        <String as FromSql>::accepts(ty)
    }
}