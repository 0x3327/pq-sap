use std::error::Error;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use pq_sap::on_chain::{rest::service::blockchain_service::{fetch_meta_address, BlockchainService}, utils::is_valid_hex};
use tokio::{self};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let theme = ColorfulTheme::default();
    
    let options = vec!["Send", "Receive"]; 
    let selection = Select::with_theme(&theme)
    .with_prompt("Select operation mode")
    .default(0)
    .items(&options)
    .interact()
    .unwrap(); 

    match  selection{
        0 => handle_send(&theme).await?, 
        1 => handle_receive(&theme).await?, 
        _ =>  unreachable!()
    }
   
    
    Ok(())
}

async fn handle_receive(theme: &ColorfulTheme) -> Result<(), Box<dyn Error>>{
    let k_hex: String = Input::with_theme(theme)
        .with_prompt("Enter spending private key(k), (hex only)")
        .validate_with(|input: &String| -> Result<(), &str> {
            if is_valid_hex(input) {
                Ok(())
            } else {
                Err("Please enter a valid hexadecimal string")
            }
        })
        .interact_text()
        .unwrap();

    let v_hex: String = Input::with_theme(theme)
        .with_prompt("Enter viewing private key(v), (hex only)")
        .validate_with(|input: &String| -> Result<(), &str> {
            if is_valid_hex(input) {
                Ok(())
            } else {
                Err("Please enter a valid hexadecimal string")
            }
        })
        .interact_text()
        .unwrap();

    let destination_wallet: String = Input::with_theme(theme)
        .with_prompt("Enter destination wallet address (hex only)")
        .validate_with(|input: &String| -> Result<(), &str> {
            if is_valid_hex(input) {
                Ok(())
            } else {
                Err("Please enter a valid hexadecimal string")
            }
        })
        .interact_text()
        .unwrap();

    let result = BlockchainService::receive_eth(&k_hex, &v_hex, &destination_wallet).await?;
    if result.len()>0{
        for r in result{
            println!("{} Received {} from stealth address {}.","SUCCESS".green(), r.1.to_string().yellow(), r.0.to_string().yellow());
        }
    }else{
        println!("{} No money on stealth addresses.", "WARNING:".yellow())
    }
   
    Ok(())
}

async fn handle_send(theme: &ColorfulTheme) -> Result<(), Box<dyn Error>>{
    let ens_domain: String = Input::with_theme(theme)
    .with_prompt("Enter ENS domain name")
    .interact_text()
    .unwrap();

    let wallet: String = Input::with_theme(theme)
        .with_prompt("Enter private key of wallet (hex only)")
        .validate_with(|input: &String| -> Result<(), &str> {
            if is_valid_hex(input) {
                Ok(())
            } else {
                Err("Please enter a valid hexadecimal string")
            }
        })
        .interact_text()
        .unwrap();
    
    let value: f64 = Input::with_theme(theme)
        .with_prompt("Enter amount to send")
        .validate_with(|input: &f64| -> Result<(), &str> {
            if *input > 0.0 {
                Ok(())
            } else {
                Err("Please enter a positive amount")
            }
        })
        .interact_text()
        .unwrap();
    

    let meta_address = fetch_meta_address(&ens_domain).await?; 

    let result = BlockchainService::send_eth(value, &wallet, &meta_address).await?;
    println!("{} Sent {} on stealth address {}", "SUCCESS:".green(), value.to_string().yellow(), result.to_string().yellow());
    Ok(())

}

