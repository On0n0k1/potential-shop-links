use crate::dao::Dao;
use crate::error::Error;
use bson::oid::ObjectId;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Document},
    Client, Collection, Database,
};
use serde::{Deserialize, Serialize};
use shuttle_runtime::SecretStore;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Entry {
    _id: ObjectId,
    user_id: ObjectId,
    data: String,
}

impl Entry {
    pub fn db_collection(dao: &Arc<Mutex<Dao>>) -> Collection<Self> {
        let dao = dao.lock().unwrap();
        let client: &Client = dao.client();
        let db: Database = client.database("entry");
        db.collection::<Entry>("entry")
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
        user_id: ObjectId,
        data: String,
    ) -> Result<Entry, Error> {
        let _id = ObjectId::new();
        let entry: Entry = Entry { _id, user_id, data };
        let collection: Collection<Entry> = Self::db_collection(dao);
        collection
            .insert_one(&entry, None)
            .await
            .or_else(Error::database)?;
        Ok(entry)
    }

    pub async fn read(dao: &Arc<Mutex<Dao>>, user_id: ObjectId) -> Result<Vec<Entry>, Error> {
        let collection: Collection<Entry> = Self::db_collection(dao);
        let filter: Document = doc! { "user_id": &user_id };
        let mut entries_cursor = collection
            .find(filter, None)
            .await
            .or_else(Error::database)?;
        // let entries: Vec<Entry> = entries.collect().await;
        let mut entries: Vec<Entry> = Vec::new();
        while let Some(Ok(entry)) = entries_cursor.next().await {
            entries.push(entry);
        }

        Ok(entries)
    }

    pub async fn delete(dao: &Arc<Mutex<Dao>>, id: ObjectId) -> Result<(), Error> {
        let collection: Collection<Entry> = Self::db_collection(dao);
        let query: Document = doc! {"_id": id};
        collection
            .delete_one(query, None)
            .await
            .or_else(Error::database)?;
        Ok(())
    }
}
