use mongodb::{
    bson::{doc, Document},
    options::IndexOptions,
    Client, Collection, Database as MongoDatabase, IndexModel,
};
use std::env;

pub mod collections {
    pub const USERS: &str = "users";
    pub const TRANSACTIONS: &str = "transactions";
    pub const ANALYSES: &str = "analyses";
    pub const PIGGY_BANKS: &str = "piggy_banks";
    pub const PROFILES: &str = "profiles";
}

/// Wrapper around MongoDB client
#[derive(Clone)]
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
            .run_command(doc! { "ping": 1 })
            .await?;

        log::info!("✅ Connected to MongoDB Atlas");

        let db = client.database("finwise");

        // Create indexes
        create_indexes(&db).await;

        Ok(Database { client, db })
    }

    pub fn collection<T>(&self, name: &str) -> Collection<T>
    where
        T: Send + Sync,
    {
        self.db.collection::<T>(name)
    }
}

/// Create required indexes (MongoDB 3.5.x compatible)
async fn create_indexes(db: &MongoDatabase) {
    let users = db.collection::<Document>(collections::USERS);
    let transactions = db.collection::<Document>(collections::TRANSACTIONS);

    // USERS: wallet_address UNIQUE
    let _ = users
        .create_index(
            IndexModel::builder()
                .keys(doc! { "wallet_address": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
        )
        .await;

    // USERS: last_active
    let _ = users
        .create_index(
            IndexModel::builder()
                .keys(doc! { "last_active": 1 })
                .build(),
        )
        .await;

    // TRANSACTIONS: wallet_address + created_at (DESC)
    let _ = transactions
        .create_index(
            IndexModel::builder()
                .keys(doc! { "wallet_address": 1, "created_at": -1 })
                .build(),
        )
        .await;

    // TRANSACTIONS: type (deposit / withdraw)
    let _ = transactions
        .create_index(
            IndexModel::builder()
                .keys(doc! { "type": 1 })
                .build(),
        )
        .await;

    log::info!("✅ Indexes created");
}