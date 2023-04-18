use std::{fs::File, io::{Write, Read}};

use tokio_postgres::{Client, types::ToSql};

pub struct DatabaseController {
    client: Client,
}

impl DatabaseController {
    ///
    /// Connects to the database
    /// 
    pub async fn connect(&mut self) { }

    ///
    /// Disconnect from the database
    /// 
    pub async fn disconnect(&mut self) { }

    ///
    /// Runs a query on the database
    /// 
    pub async fn query(
        &mut self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)]
        ) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::Error>
    {
        self.client.query(statement, params).await
    }

    ///
    /// Executes a statement on the database, returning the number of rows affected
    /// 
    pub async fn excute(
        &mut self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)]
        ) -> Result<u64, tokio_postgres::Error>
    {
        self.client.execute(statement, params).await
    }

    ///
    /// Stores data on the file system
    /// 
    pub async fn store_in_file(
        &mut self,
        file_path: &str,
        data: Vec<u8>) -> std::io::Result<()>
    {
        let mut file = File::create(file_path)?;

        file.write_all(&data)
    }

    ///
    /// Loads data from the file system
    /// 
    pub async fn load_from_file(
        &mut self,
        file_path: &str) -> std::io::Result<Vec<u8>>
    {
        let mut file = File::open(file_path)?;
        let mut data = Vec::new();

        file.read_to_end(&mut data)?;
        Ok(data)
    }
}