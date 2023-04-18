macro_rules! uuid_based {
    ($name:ident) => {
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, postgres_types::FromSql, postgres_types::ToSql)]
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
    };
}