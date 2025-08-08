use tracing::{info, error};
use std::process::ExitCode;
use clap::Parser;
use simular_core::{BaseEvm, ProofVerificationResult};
use alloy_primitives::{U256, Address};
use std::str::FromStr;
use serde::{Serialize, Deserialize};
use std::fs;
use ethers_core::types::{Block, TxHash};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,         // Input JSON file path from host
}

/// Data structure saved by the host
#[derive(Serialize, Deserialize)]
struct HostData {
    evm_json: String,
    sender: String,
    recipient: String,
    eth_amt: f64,
    sender_initial_balance: String,
    recipient_initial_balance: String,
    block_data: String, // Serialized block data for proof verification
}

fn run() -> Result<()> {
    let args = Args::parse();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();
    info!("Running ETH transfer enclave (offline simulation)...");

    // Read the JSON file from host
    info!("Reading host data from: {}", args.input);
    let json_data = fs::read_to_string(&args.input)?;
    let host_data: HostData = serde_json::from_str(&json_data)?;

    // Parse addresses
    let sender = Address::from_str(&host_data.sender)
        .map_err(|e| format!("Invalid sender address: {}", e))?;
    let recipient = Address::from_str(&host_data.recipient)
        .map_err(|e| format!("Invalid recipient address: {}", e))?;

    info!("Sender address: {}", sender);
    info!("Recipient address: {}", recipient);
    info!("Amount to transfer: {} ETH", host_data.eth_amt);
    info!("Initial sender balance: {} ETH", 
          format_ether(U256::from_str(&host_data.sender_initial_balance)?));
    info!("Initial recipient balance: {} ETH", 
          format_ether(U256::from_str(&host_data.recipient_initial_balance)?));

    // Restore EVM from JSON
    info!("Restoring EVM state from serialized data...");
    let mut evm = BaseEvm::from_json(&host_data.evm_json)?;

    // Verify initial balances match what the host recorded
    let current_sender_balance = evm.get_balance(sender)?;
    let current_recipient_balance = evm.get_balance(recipient)?;
    
    let expected_sender = U256::from_str(&host_data.sender_initial_balance)?;
    let expected_recipient = U256::from_str(&host_data.recipient_initial_balance)?;
    
    if current_sender_balance != expected_sender {
        return Err(format!("Sender balance mismatch after restore! Expected: {}, Got: {}", 
                          expected_sender, current_sender_balance).into());
    }
    if current_recipient_balance != expected_recipient {
        return Err(format!("Recipient balance mismatch after restore! Expected: {}, Got: {}", 
                          expected_recipient, current_recipient_balance).into());
    }
    
    info!("✅ Balance verification passed - EVM state restored correctly");

    // Deserialize block data for proof verification
    info!("Deserializing block data for proof verification...");
    let block: Block<TxHash> = serde_json::from_str(&host_data.block_data)?;
    
    // Verify account proofs in the database
    info!("Verifying account proofs in the enclave (offline)...");
    let verification_results = evm.verify_proofs(&block)?;
    
    if verification_results.is_empty() {
        info!("No account proofs found in database to verify");
    } else {
        info!("Account proof verification results:");
        let mut all_valid = true;
        for (address, result) in &verification_results {
            match result {
                ProofVerificationResult::Valid => {
                    info!("✅ Address {}: proof valid", address);
                }
                ProofVerificationResult::Invalid => {
                    error!("❌ Address {}: proof invalid", address);
                    all_valid = false;
                }
            }
        }
        
        if !all_valid {
            return Err("❌ One or more account proofs are invalid! Cannot proceed with simulation.".into());
        }
        
        info!("✅ All account proofs verified successfully!");
    }

    // Perform the transfer simulation
    let wei_amount = (host_data.eth_amt * 1e18) as u128;
    let transfer_amount = U256::from(wei_amount);
    info!("Simulating transfer of {} ETH ({} wei)...", host_data.eth_amt, wei_amount);

    evm.transfer(sender, recipient, transfer_amount)?;

    // Check final balances
    let sender_final_balance = evm.get_balance(sender)?;
    let recipient_final_balance = evm.get_balance(recipient)?;

    info!("✅ Transfer simulation completed!");
    info!("Sender's final balance: {} ETH", format_ether(sender_final_balance));
    info!("Recipient's final balance: {} ETH", format_ether(recipient_final_balance));

    // Verify the transfer worked correctly
    let expected_sender_final = expected_sender - transfer_amount;
    let expected_recipient_final = expected_recipient + transfer_amount;
    
    if sender_final_balance != expected_sender_final {
        return Err(format!("Sender final balance incorrect! Expected: {}, Got: {}", 
                          expected_sender_final, sender_final_balance).into());
    }
    if recipient_final_balance != expected_recipient_final {
        return Err(format!("Recipient final balance incorrect! Expected: {}, Got: {}", 
                          expected_recipient_final, recipient_final_balance).into());
    }

    // Calculate and display the changes
    let sender_change = expected_sender - sender_final_balance;
    let recipient_change = recipient_final_balance - expected_recipient;

    info!("📊 Simulation Results:");
    info!("   Sender balance change: -{} ETH", format_ether(sender_change));
    info!("   Recipient balance change: +{} ETH", format_ether(recipient_change));
    info!("   Transfer amount verified: {} ETH", format_ether(transfer_amount));
    
    if sender_change == transfer_amount && recipient_change == transfer_amount {
        info!("✅ All balance changes are correct!");
    } else {
        return Err("❌ Balance change verification failed!".into());
    }

    info!("Enclave simulation completed successfully!");
    Ok(())
}

fn format_ether(wei: U256) -> String {
    let ether_divisor = U256::from(1e18 as u64);
    let ether_whole = wei / ether_divisor;
    let ether_remainder = wei % ether_divisor;
    
    // Convert remainder to fractional part with 18 decimal places
    let fractional_str = format!("{:018}", ether_remainder);
    // Remove trailing zeros
    let fractional_trimmed = fractional_str.trim_end_matches('0');
    
    if fractional_trimmed.is_empty() {
        format!("{}.0", ether_whole)
    } else {
        format!("{}.{}", ether_whole, fractional_trimmed)
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            error!("Enclave failed: {}", e);
            ExitCode::FAILURE
        }
    }
}