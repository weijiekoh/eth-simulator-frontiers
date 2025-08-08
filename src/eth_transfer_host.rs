use tracing::{info, error};
use std::process::ExitCode;
use clap::Parser;
use simular_core::{BaseEvm, CreateFork, ProofVerificationResult};
use alloy_primitives::{U256, Address};
use std::str::FromStr;
use ethers_providers::{Provider, Http, Middleware};
use ethers_core::types::{H256, BlockId, U64};
use serde::{Serialize, Deserialize};
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    sender: String,         // Sender's address

    #[arg(short, long)]
    recipient: String,      // Recipient's address

    #[arg(short, long)]
    eth_amt: f64,           // Amount of ETH to send

    #[arg(short, long)]
    output: String,         // Output JSON file path
}

/// Data structure to save for the enclave
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

async fn run() -> Result<()> {
    let args = Args::parse();
    
    // Initialize tracing
    tracing_subscriber::fmt::init();
    info!("Running forked ETH transfer host...");

    // Parse addresses from CLI args
    let sender = Address::from_str(&args.sender)
        .map_err(|e| format!("Invalid sender address: {}", e))?;
    let recipient = Address::from_str(&args.recipient)
        .map_err(|e| format!("Invalid recipient address: {}", e))?;

    let rpc_url = "https://eth.llamarpc.com";
    info!("RPC URL: {}", rpc_url);
    info!("Sender address: {}", sender);
    info!("Recipient address: {}", recipient);
    info!("Amount to transfer: {} ETH", args.eth_amt);

    // Create a fork from the specified RPC
    let fork_info = CreateFork::latest_block(rpc_url.to_string());
    let mut evm = BaseEvm::new(Some(fork_info));

    // Get block information
    let block_number = evm.get_block_number();
    let block_hash = evm.get_current_block_hash()?;
    
    info!("Block number: {}", block_number);
    info!("Block hash: {}", block_hash);

    // Fetch block header and verify hash matches
    info!("Fetching block header to verify hash...");
    
    // Create provider to fetch block header
    let provider = Provider::<Http>::try_from(rpc_url)?;
    
    // Fetch the block header using block number
    let block = provider
        .get_block(BlockId::from(U64::from(block_number)))
        .await
        .map_err(|e| format!("Failed to get block: {}", e))?
        .ok_or("Block not found")?;
    
    // Verify that the block hash matches what we got from the EVM
    let expected_block_hash = H256::from(block_hash.0);
    if block.hash != Some(expected_block_hash) {
        return Err(format!(
            "Block hash mismatch! Expected: {}, Got: {:?}", 
            expected_block_hash,
            block.hash
        ).into());
    }
    
    info!("Block hash verification passed: {}", expected_block_hash);

    // Check the initial balances from the forked chain
    let sender_balance = evm.get_balance(sender)?;
    let recipient_balance = evm.get_balance(recipient)?;
    
    info!("Sender's initial balance: {} ETH", format_ether(sender_balance));
    info!("Recipient's initial balance: {} ETH", format_ether(recipient_balance));

    // Verify account proofs in the database
    info!("Verifying account proofs in the database...");
    let verification_results = evm.verify_proofs(&block)?;
    
    if verification_results.is_empty() {
        info!("No account proofs found in database to verify");
    } else {
        info!("Account proof verification results:");
        for (address, result) in verification_results {
            match result {
                ProofVerificationResult::Valid => {
                    info!("✅ Address {}: proof valid", address);
                }
                ProofVerificationResult::Invalid => {
                    info!("❌ Address {}: proof invalid", address);
                }
            }
        }
    }

    // Serialize the EVM state
    info!("Serializing EVM state...");
    let evm_json = evm.to_json()?;
    
    // Serialize block data for enclave
    let block_json = serde_json::to_string(&block)?;
    
    // Create data structure to save
    let host_data = HostData {
        evm_json,
        sender: args.sender,
        recipient: args.recipient,
        eth_amt: args.eth_amt,
        sender_initial_balance: sender_balance.to_string(),
        recipient_initial_balance: recipient_balance.to_string(),
        block_data: block_json,
    };

    // Save to JSON file
    let json_data = serde_json::to_string_pretty(&host_data)?;
    fs::write(&args.output, json_data)?;
    
    info!("EVM state and transaction data saved to: {}", args.output);
    info!("Host processing completed successfully!");

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

#[tokio::main]
async fn main() -> ExitCode {
    match run().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            error!("Host failed: {}", e);
            ExitCode::FAILURE
        }
    }
}