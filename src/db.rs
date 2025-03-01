use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, SqlitePool};

use directories::ProjectDirs;
use std::path::PathBuf;
use tokio::fs::{self, File};

static MIGRATOR: Migrator = sqlx::migrate!();

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db_path = Self::get_database_path();

        let database_url = format!("sqlite://{}", db_path.await.display());

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        MIGRATOR.run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn get_database_path() -> PathBuf {
        let dirs =
            ProjectDirs::from("", "", "rustodoo").expect("Failed to determine project directories");

        let db_dir = dirs.data_local_dir();

        if !db_dir.exists() {
            fs::create_dir_all(db_dir)
                .await
                .expect("Failed to create database directory");
        }

        let db_path = db_dir.join("todos.db");

        if !db_path.exists() {
            File::create(&db_path)
                .await
                .expect("Failed to create database file");

            println!("Created new SQLite database at {:?}", db_path); // Optional log for clarity
        }

        db_path
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
