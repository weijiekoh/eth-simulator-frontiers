#!/bin/bash

# Default values
SENDER="0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"
RECIPIENT="0x1111000000000000000000000000000000000001"
ETH_AMT="1.0"
OUTPUT_FILE="/tmp/transfer_data.json"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -s|--sender)
            SENDER="$2"
            shift 2
            ;;
        -r|--recipient)
            RECIPIENT="$2"
            shift 2
            ;;
        -a|--eth-amt)
            ETH_AMT="$2"
            shift 2
            ;;
        -o|--output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [options]"
            echo "Options:"
            echo "  -s, --sender ADDRESS        Sender's Ethereum address (default: $SENDER)"
            echo "  -r, --recipient ADDRESS     Recipient's Ethereum address (default: $RECIPIENT)"
            echo "  -a, --eth-amt AMOUNT        Amount of ETH to transfer (default: $ETH_AMT)"
            echo "  -o, --output FILE           Output file path (default: $OUTPUT_FILE)"
            echo "  -h, --help                  Show this help message"
            echo ""
            echo "Example:"
            echo "  $0 --sender 0x123... --recipient 0x456... --eth-amt 2.5"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

set -e  # Exit on any error

echo "🔧 ETH Transfer Host/Enclave Demo"
echo "================================="
echo "Sender: $SENDER"
echo "Recipient: $RECIPIENT"
echo "Amount: $ETH_AMT ETH"
echo "Data file: $OUTPUT_FILE"
echo ""

echo "📡 Step 1: Running host to fetch state from Ethereum network..."
echo "---------------------------------------------------------------"
cargo run --bin eth_transfer_host -- \
    --sender "$SENDER" \
    --recipient "$RECIPIENT" \
    --eth-amt "$ETH_AMT" \
    --output "$OUTPUT_FILE"

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Host completed successfully!"
    echo "📁 EVM state saved to: $OUTPUT_FILE"
    echo ""
else
    echo "❌ Host failed!"
    exit 1
fi

echo "🔒 Step 2: Running enclave for offline simulation..."
echo "---------------------------------------------------"
cargo run --bin eth_transfer_enclave -- --input "$OUTPUT_FILE"

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Enclave simulation completed successfully!"
    echo ""
    echo "🎉 Host/Enclave workflow completed!"
    echo "   • Host fetched real state from Ethereum"
    echo "   • Enclave verified proofs and simulated transaction offline"
else
    echo "❌ Enclave failed!"
    exit 1
fi