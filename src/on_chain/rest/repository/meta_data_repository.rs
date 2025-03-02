use sqlx::{MySqlPool, query, query_as};
use crate::on_chain::rest::model::meta_data::MetaData;

pub struct MetaDataRepository {
    pool: MySqlPool, 
}

impl MetaDataRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub async fn insert_meta_data_entry(&self, wallet: &str, last_block: u64) -> Result<(), sqlx::Error> {
        query!(
            "INSERT INTO meta_data (wallet, last_block) VALUES (?, ?) ON DUPLICATE KEY UPDATE last_block = ?",
            wallet,
            last_block,
            last_block
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_meta_data_by_wallet(&self, wallet: &str) -> Result<Option<MetaData>, sqlx::Error> {
        let result = query_as!(MetaData, "SELECT wallet, last_block FROM meta_data WHERE wallet = ?", wallet)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }
}
