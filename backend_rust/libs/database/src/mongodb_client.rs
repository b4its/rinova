use mongodb::{Client, options::ClientOptions};
use std::env;

pub type MongoClient = Client;

pub async fn create_mongo_client() -> Result<MongoClient, mongodb::error::Error> {
    let mongo_uri = env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://rinova:rinova_secret@localhost:27017/rinova?authSource=admin".to_string());

    let client_options = ClientOptions::parse(&mongo_uri).await?;
    
    Client::with_options(client_options)
}

pub fn get_database(client: &MongoClient) -> mongodb::Database {
    let db_name = env::var("MONGO_DB_NAME")
        .unwrap_or_else(|_| "rinova".to_string());
    
    client.database(&db_name)
}
