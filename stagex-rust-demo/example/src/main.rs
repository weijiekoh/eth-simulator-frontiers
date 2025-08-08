use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct Message {
    greeting: String,
    target: String,
}

fn main() {
    println!("Hello from StageX-built Rust binary!");
    
    let args: Vec<String> = env::args().collect();
    let target = if args.len() > 1 {
        args[1].clone()
    } else {
        "World".to_string()
    };
    
    let message = Message {
        greeting: "Hello".to_string(),
        target,
    };
    
    println!("Message: {}", serde_json::to_string_pretty(&message).unwrap());
    println!("Built with musl and statically linked!");
    
    // Display build info
    println!("Built for target: {}", env!("TARGET"));
    println!("Rust version: {}", env!("RUSTC_VERSION"));
}