use std::sync::Arc;

use actix_web::{post, web::{self, Json}, HttpResponse, Responder};
use ethers::abi::Address;
use pqc_kyber::KYBER_SECRETKEYBYTES;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{on_chain::rest::service::blockchain_service::BlockchainService, recipient::recipient::{scan, RecipientInputData}, sender::sender::{send, SenderInputData}};


#[post("send-eth")]
async fn send_eth(data: Json<SenderInputData>) -> impl Responder{
    match send(&data){
        Ok(result) =>{
            let json_response = json!({
                "stealth_address":result.0, 
                "ephemeral_public_key": hex::encode(result.1), 
                "viewtag": hex::encode(result.2)
            }); 
            HttpResponse::Ok().json(json_response)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

#[post("scan-eth")]
async fn scan_eth(service: web::Data<Arc<BlockchainService>>, data: Json<ScanRequest>) -> impl Responder{
    if &data.v.len()/2 != KYBER_SECRETKEYBYTES { 
        return HttpResponse::BadRequest().body(format!(
            "Error: 'v' must be exactly {} bytes long", KYBER_SECRETKEYBYTES
        ));
    }
   
    match service.fetch_transactions(&data.destination_wallet).await{
        Ok(result) => {
            let recipient_input_data: RecipientInputData = RecipientInputData{
                stealth_addresses: result.0,
                ephemeral_pub_key_reg: result.1,
                view_tags: result.2,
                k_pk: data.k_pub.clone(), 
                v: data.v.clone(),
            }; 
            match scan(recipient_input_data){
                Ok(result) =>{
                    let json_response: Vec<_> = result.0.iter().zip(result.1).map(|(shared_secret, stealth_address)| {
                        json!({
                            "shared_secret": hex::encode(shared_secret),
                            "stealth_address": stealth_address
                        })
                    }).collect();
            
                    HttpResponse::Ok().json(json_response)
                }
                Err(e) => {
                    HttpResponse::InternalServerError().body(format!("Error: {}", e))
                }
            }
        }
        Err(_) => todo!(),
    }

}

#[derive(Deserialize, Serialize)]
pub struct ScanRequest{
    pub k_pub: String, 
    pub v: String, 
    pub destination_wallet: String, 
}

#[derive(Deserialize, Serialize)]
pub struct ScanEntry{
    pub shared_secret: String,
    pub stealth_address: Address 
}

#[derive(Deserialize, Debug)]
pub struct SendEthResponse {
    pub stealth_address: Address,
    pub ephemeral_public_key: String,
    pub viewtag: String,
}