// Copyright 2025 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;
use groth_16_verifier::Proof;

pub mod estop;
pub mod router;
pub mod state;
use estop::*;
pub use ownable::OwnableError;
use router::*;
use state::*;

declare_id!("3uqcghDkzREcs1nbUk1LBXztaHCr9Abt9nrPfv1modF3");

/// Verifier Router Program for Anchor
///
/// This program provides a routing and management system for zero-knowledge proof verifiers
/// on Solana. It implements a registry of verifier programs that can be dynamically added
/// and managed, with built-in emergency stop capabilities for security.
///
/// # Features
/// * Registry of verifier programs with unique selectors
/// * Two-step ownership transfers via the Ownable trait
/// * Verifier upgrade authority checks on addition to registry
/// * Emergency stop functionality with proof-based or owner-based calls
#[program]
pub mod verifier_router {
    use state::VerifierRouter;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        router::initialize(ctx)
    }

    pub fn add_verifier(ctx: Context<AddVerifier>, selector: u32) -> Result<()> {
        // This function checks ownership and can only be called by the owner
        router::add_verifier(ctx, selector)
    }

    pub fn verify(
        ctx: Context<Verify>,
        _selector: u32,
        proof: Proof,
        image_id: [u8; 32],
        journal_digest: [u8; 32],
    ) -> Result<()> {
        router::verify(ctx, proof, image_id, journal_digest)
    }

    pub fn emergency_stop(ctx: Context<EmergencyStop>, selector: u32) -> Result<()> {
        // This function checks ownership and can only be called by the owner
        estop::emergency_stop_by_owner(ctx, selector)
    }

    pub fn emergency_stop_with_proof(
        ctx: Context<EmergencyStop>,
        selector: u32,
        proof: Proof,
    ) -> Result<()> {
        estop::emergency_stop_with_proof(ctx, selector, proof)
    }

    pub fn transfer_ownership(
        ctx: Context<VerifierRouterTransferOwnership>,
        new_owner: Pubkey,
    ) -> Result<()> {
        // This function checks ownership and can only be called by the owner
        VerifierRouter::transfer_ownership(ctx, new_owner)
    }

    pub fn accept_ownership(ctx: Context<VerifierRouterAcceptOwnership>) -> Result<()> {
        // This function can only be called by the pending owner
        VerifierRouter::accept_ownership(ctx)
    }

    pub fn cancel_transfer(ctx: Context<VerifierRouterCancelTransfer>) -> Result<()> {
        // This function checks ownership and can only be called by the owner or pending owner
        VerifierRouter::cancel_transfer(ctx)
    }

    pub fn renounce_ownership(ctx: Context<VerifierRouterRenounceOwnership>) -> Result<()> {
        // This function checks ownership and can only be called by the owner
        VerifierRouter::renounce_ownership(ctx)
    }
}
