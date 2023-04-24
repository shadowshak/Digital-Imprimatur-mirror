use serde::{Serialize, Deserialize};
use uuid::Uuid;

macro_rules! uuid_based {
    ($name:ident) => {
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, postgres_types::FromSql, postgres_types::ToSql)]
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

    }
}