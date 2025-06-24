# RISC Zero Solana

> [!WARNING]
> This is unaudited and not yet recommended for production use.

[RISC Zero] is a zero-knowledge verifiable general computing platform, with [Solana] integration.
This repository contains our Solana verifier infrastructure and supporting code.

## Components

### Verifier Router

Our core verifier infrastructure allows secure on-chain verification of RISC Zero zkVM proofs. The router provides:

- On-chain verification of Groth16 proofs from the RISC Zero zkVM
- Dynamic verifier routing and management
- Emergency stop functionality for security

### Supporting Components

We provide several supporting libraries to help developers integrate with the verifier:

- [Groth16 Verifier][verifier]: Groth16 proof verification using Solana `alt-bn254` system calls
- [Ownable Library][ownable]: Secure two-step ownership transfer for Solana programs

## Getting Started

Full documentation for using RISC Zero, including guides for [writing zkVM programs][risc0-quickstart], and more can be found at our [Developer Documentation].

## Quick Links

- [Groth16 Verifier Program][verifier]
- [Verifier Router][router]
- [Ownable Library][ownable]

[RISC Zero]: https://github.com/risc0/risc0
[Solana]: https://solana.com
[router]: ./solana-verifier
[ownable]: ./solana-ownable
[verifier]: ./solana-verifier/programs/groth_16_verifier
[risc0-quickstart]: https://dev.risczero.com/api/zkvm/quickstart
[Developer Documentation]: https://dev.risczero.com
