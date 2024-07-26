pub mod entry;
pub mod user;

use crate::error::Error;
use mongodb::{options::ClientOptions, Client};

pub const DBPATH: &str = "./data/users.db";

pub struct Dao {
    client: Client,
}

impl Dao {
    pub async fn new(secrets: &shuttle_runtime::SecretStore) -> Result<Self, Error> {
        let uri = secrets.get("URI").unwrap();
        // let uri = crate::globals::MONGODBURI;
        let options = ClientOptions::parse(uri).await.unwrap();
        let client = Client::with_options(options).or_else(Error::database)?;
        Ok(Self { client })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
