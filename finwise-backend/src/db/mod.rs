use mongodb::{Client, Collection, Database as MongoDatabase};
use std::env;

pub mod collections {
    pub const USERS: &str = "users";
    pub const TRANSACTIONS: &str = "transactions";
    pub const ANALYSES: &str = "analyses";
    pub const PIGGY_BANKS: &str = "piggy_banks";
    pub const PROFILES: &str = "profiles";
}

/// Wrapper around MongoDB client
pub struct Database {
    pub client: Client,
    pub db: MongoDatabase,
}

impl Database {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
        let client = Client::with_uri_str(&uri).await?;

        // Ping to verify connection
        client
            .database("admin")
            .run_command(mongodb::bson::doc! { "ping": 1 })
            .await?;

        log::info!("✅ Connected to MongoDB Atlas");
        let db = client.database("finwise");

        Ok(Database { client, db })
    }

    pub fn collection<T>(&self, name: &str) -> Collection<T>
    where
        T: Send + Sync,
    {
        self.db.collection::<T>(name)
    }
}