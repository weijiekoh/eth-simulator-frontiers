# eth-simulator

This is a hackerthon project for [Paradigm Frontiers 2025](https://frontiers.paradigm.xyz/).

Slides [here](https://docs.google.com/presentation/d/132xQpIaAkmjIevrJWtrOgLIvlJhBo0iPk0bavLc6Xa8/).

## Motivation

A major hack occured in February 2025 where an attacker compromised a popular
multisig wallet user interface and targeted a centralised exchange. The
attacker was able to trick employees into signing a malicious transaction that
moved a large amount of funds into their own wallet. They did this by injecting
Javascript into the UI that modified the transaction calldata. The signers did
not fully verify what they signed, so the malicious transaction was approved
and led to a significant loss of funds.

In short, it is very risky to rely on wallet UIs even if one uses a multisig or
hardware wallets. Good operational security must incorporate independent ways
to verify transaction calldata.

## Solution

This project offers a new way to independently verify transaction calldata: a
*securely built* transaction simulator that runs in a Trusted Execution
Environment (TEE).

An attacker would have to compromise *both* the TEE and the multisig UI to successfully
trick the user into signing a malicious transaction. This is harder than only hacking the UI.

Additionally, a TEE can attest to the fact that it is running the correct
transaction simulation executable. The user can verify this attestation and
this provides additional assurance.

Finally, it is cruical that the simulator binary is built in a secure way. This
means that the compiler that builds the binary, the compiler that compiles the
compiler, and so on must be securely built. This is a hard problem but the
[StageX](https://stagex.tools) toolchain can help with this.

## Demo

The current iteration of this project only supports simulating plain Ethereum
transfers. Future versions will easily support arbitrary transactions, such as
multisig transactions, ERC20 transfers, and more.

Run:

```bash
cargo build && \
./end_to_end_test_eth_transfer.sh
```

## Trust assumptions

Like all TEE-based solutions, users must trust the hardware manufacturer and operators.

## The current implementation

This project was built in less than 24 hours during the [Paradigm Frontiers
2025](https://frontiers.paradigm.xyz/) hackathon. It is a proof of concept and
must not be used in production. Due to time constraints, some features are
incomplete.

I attepted to use StageX to build a reproducible enclave binary, but dependency
problems prevented me from succeeding in time. It appears that the `zstd`
library is not yet available as a StageX package. Nevertheless, I included the
Containerfile and Makefile I used for future reference.
