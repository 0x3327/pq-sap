use sqlx::MySqlPool;

pub fn is_valid_hex(hex_str: &String) -> bool{
    let trimmed = if hex_str.starts_with("0x") { &hex_str[2..] } else { &hex_str[..] };
    trimmed.chars().all(|c| c.is_digit(16)) && trimmed.len()%2 == 0
}

pub async fn create_metadata_table(pool: &MySqlPool) -> Result<(), sqlx::Error>{
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS meta_data (
            wallet VARCHAR(42) PRIMARY KEY,
            last_block BIGINT UNSIGNED NOT NULL
        )"
    )
    .execute(pool)
    .await?;

    Ok(())
}