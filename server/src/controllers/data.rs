use std::{fs::File, io::{Write, Read}, error::Error};

use tokio_postgres::{Client, types::ToSql, NoTls};

pub struct DatabaseController {
    client: Option<Client>,
}

impl DatabaseController {
    ///
    /// Connects to the database
    /// 
    pub async fn connect(&mut self) -> Result<(), Box<dyn Error>> {
        assert!(self.client.is_none());

        let connection_string = "host=localhost user=jkpalladino dbname=di";

        let (client, connection)
            = match tokio_postgres::connect(connection_string, NoTls).await
        {
            Ok(cc) => cc,
            Err(e) => {
                eprintln!("{e:#?}");
                return Err(Box::new(e));
            }
        };

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        _ = self.client.insert(client);

        Ok(())
    }

    ///
    /// Disconnect from the database
    /// 
    pub async fn disconnect(&mut self) {
        self.client.take();
    }

    ///
    /// Runs a query on the database
    /// 
    pub async fn query(
        &mut self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)]
        ) -> Result<Vec<tokio_postgres::Row>, Box<dyn Error>>
    {
        if self.client.is_none() {
            self.connect().await?;
        }

        if let Some(client) = &mut self.client {
            match client.query(statement, params).await {
                Ok(rows) => return Ok(rows),
                Err(e) => {
                    eprintln!("{e:#?}");

                    return Err(Box::new(e));
                }
            }
        }

        else {
            eprintln!("No connection");
            return Err("No database connection".into())
        }
    }

    ///
    /// Executes a statement on the database, returning the number of rows affected
    /// 
    pub async fn excute(
        &mut self,
        statement: &str,
        params: &[&(dyn ToSql + Sync)]
        ) -> Result<u64, Box<dyn Error>>
    {
        if self.client.is_none() {
            self.connect().await?;
        }

        if let Some(client) = &mut self.client {
            match client.execute(statement, params).await {
                Ok(rows) => return Ok(rows),
                Err(e) => {
                    eprintln!("{e:#?}");

                    return Err(Box::new(e));
                }
            }
        }

        else {
            eprintln!("No connection");
            return Err("No database connection".into())
        }
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

impl Default for DatabaseController {
    fn default() -> Self {
        Self { client: None }
    }
}