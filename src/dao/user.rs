use crate::dao::Dao;
use crate::error::Error;
use bson::oid::ObjectId;
use mongodb::{
    bson::{doc, Document},
    Client, Collection, Database,
};
use serde::{Deserialize, Serialize};
use shuttle_runtime::SecretStore;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    _id: ObjectId,
    username: String,
    password: String,
}

impl User {
    pub fn db_collection(dao: &Arc<Mutex<Dao>>) -> Collection<Self> {
        let dao = dao.lock().unwrap();
        let client: &Client = dao.client();
        let db: Database = client.database("user");
        db.collection::<User>("user")
    }

    pub async fn reset(dao: &Arc<Mutex<Dao>>, secrets: &SecretStore) -> Result<(), Error> {
        let username = secrets.get("USERNAME").unwrap();
        let collection = Self::db_collection(dao);
        // 3. Check if the row exists
        // let filter = doc! { "username": "admin" };
        let filter = doc! { "username": &username };
        if collection
            .find_one(filter.clone(), None)
            .await
            .or_else(Error::database)?
            .is_some()
        {
            // 4. Delete the row if it exists
            collection
                // .delete_one(doc! { "username": "admin" }, None)
                .delete_one(doc! { "username": &username }, None)
                .await
                .or_else(Error::database)?;
        }
        Ok(())
    }

    pub async fn create(
        dao: &Arc<Mutex<Dao>>,
        username: String,
        password: String,
    ) -> Result<User, Error> {
        let _id = ObjectId::new();
        let user: User = User {
            _id,
            username,
            password,
        };
        let collection: Collection<User> = Self::db_collection(dao);
        collection
            .insert_one(&user, None)
            .await
            .or_else(Error::database)?;
        Ok(user)
    }

    pub async fn authenticate(
        dao: &Arc<Mutex<Dao>>,
        username: String,
        password: String,
    ) -> Result<(), Error> {
        let collection: Collection<User> = Self::db_collection(dao);
        let filter: Document = doc! { "username": &username };
        let user: Option<User> = collection
            .find_one(filter, None)
            .await
            .or_else(Error::database)?;
        match user {
            None => Err(Error::UserNotFound),
            Some(user) => {
                if user.password.eq(&password) {
                    Ok(())
                } else {
                    Error::incorrect_password()
                }
            }
        }
    }
}
