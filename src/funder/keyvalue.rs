use bdk::{database::SqliteDatabase, rusqlite};
use color_eyre::eyre::{self, Context};

pub struct KeyValueStorage<'a> {
    db: &'a SqliteDatabase,
}

impl<'a> KeyValueStorage<'a> {
    pub fn new(db: &'a SqliteDatabase) -> Self {
        Self { db }
    }

    /// Get value for key.
    pub(crate) fn private_key(&self) -> eyre::Result<Option<String>> {
        self.create_key_value_table()?;

        let result = self.db.connection.query_row(
            "SELECT value FROM faucet_key_value WHERE key = 'private_key'",
            [],
            |row| row.get(0),
        );

        match result {
            Ok(key) => Ok(key),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e).wrap_err("failed to get private key"),
        }
    }

    pub(crate) fn insert_private_key(&self, private_key: &str) -> eyre::Result<()> {
        self.create_key_value_table()?;

        self.db
            .connection
            .execute(
                "INSERT OR REPLACE INTO faucet_key_value (key, value) VALUES ('private_key', ?)",
                [private_key],
            )
            .wrap_err("failed to insert private key")?;

        Ok(())
    }

    /// Create table for key values if not exists.
    fn create_key_value_table(&self) -> eyre::Result<()> {
        self.db
            .connection
            .execute(
                "CREATE TABLE IF NOT EXISTS faucet_key_value (
                key TEXT PRIMARY KEY,
                value TEXT
            )",
                [],
            )
            .wrap_err("failed to create table for key values")?;

        Ok(())
    }
}
