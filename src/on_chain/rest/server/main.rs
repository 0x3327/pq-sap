use actix_web::{web, App, HttpServer};
use pq_sap::on_chain::{rest::{controller::blockchain_controller::{receive_eth, send_eth}, repository::meta_data_repository::MetaDataRepository, service::blockchain_service::BlockchainService}, utils::create_metadata_table};
use sqlx::MySqlPool;
use std::{env, sync::Arc}; 
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok(); 
    let db_url = env::var("DATABASE_URL").expect("Database url not set.");
    let connection_string = env::var("CONNECTION_STRING").expect("Incorrect connection string.");
    let pool = MySqlPool::connect(&db_url)
    .await
    .expect("Failed to connect to db."); 

    create_metadata_table(&pool).await.expect("Failed to create the table"); 

    let meta_data_repo = MetaDataRepository::new(pool.clone()); 
    let blockchain_service = Arc::new(BlockchainService::new(meta_data_repo));
         
    HttpServer::new(move ||{
        App::new()
        .app_data(web::Data::new(blockchain_service.clone()))
        .service(send_eth)
        .service(receive_eth)
    })
    .bind(connection_string)?
    .run()
    .await
}

