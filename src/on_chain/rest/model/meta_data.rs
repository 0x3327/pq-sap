use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct MetaData {
    pub wallet: String,  
    pub last_block: u64, 
}