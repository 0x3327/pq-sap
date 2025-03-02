use std::sync::Arc;

use actix_web::{post, web::{self, Json}, HttpResponse, Responder};
use pqc_kyber::KYBER_SECRETKEYBYTES;
use serde::Deserialize;
use serde_json::json;
use crate::on_chain::rest::service::blockchain_service::BlockchainService;


#[post("send-eth")]
async fn send_eth(data: Json<SendEthRequest>) -> impl Responder{
    match BlockchainService::send_eth(data.value, &data.wallet_sk, &data.ens_name).await{
        Ok((stealth_address, tx_hash)) =>{
            HttpResponse::Ok().json(json!({
                "stealth_address": stealth_address, 
                "tx_hash": tx_hash
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[post("receive-eth")]
async fn receive_eth(service: web::Data<Arc<BlockchainService>>, data: Json<ReceiveEthRequest>) -> impl Responder{
    if data.v.len()/2 != KYBER_SECRETKEYBYTES { 
        return HttpResponse::BadRequest().body(format!(
            "Error: 'v' must be exactly {} bytes long", KYBER_SECRETKEYBYTES
        ));
    }
    if data.k.len()/2 != 32 { 
        return HttpResponse::BadRequest().body(format!(
            "Error: 'k' must be exactly {} bytes long", 32
        ));
    }

    match service.receive_eth(&data.k, &data.v, &data.destination_wallet).await{
        Ok(result) =>{
            let json_response: Vec<_> = result.iter().map(|(stealth_address, value)| {
                json!({
                    "stealth_address": stealth_address,
                    "value": value
                })
            }).collect();
    
            HttpResponse::Ok().json(json_response)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[derive(Deserialize)]
struct SendEthRequest{
    value: f64, 
    wallet_sk: String, 
    ens_name: String, 
}

#[derive(Deserialize)]
struct ReceiveEthRequest{
    k: String, 
    v: String, 
    destination_wallet: String, 
}