use postgres_types::{FromSql, ToSql};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

macro_rules! uuid_based {
    ($name:ident) => {
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct $name {
    id: uuid::Uuid,
}

impl $name {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
        }
    }
}

impl serde::Serialize for $name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        self.id.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for $name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let id = uuid::Uuid::deserialize(deserializer)?;

        Ok($name { id })
    }
}


impl<'a> postgres_types::FromSql<'a> for $name {
    fn from_sql(ty: &postgres_types::Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let id = uuid::Uuid::from_sql(ty, raw)?;

        Ok($name { id })
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        postgres_types::Type::UUID == *ty
    }
}

impl postgres_types::ToSql for $name {
    fn to_sql(&self, ty: &postgres_types::Type, out: &mut postgres_types::private::BytesMut) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
        where Self: Sized
    {
        self.id.to_sql(ty, out)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
        where Self: Sized
    {
        <uuid::Uuid as postgres_types::ToSql>::accepts(ty)
    }

    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    {
        self.id.to_sql_checked(ty, out)
    }
}

    }
}