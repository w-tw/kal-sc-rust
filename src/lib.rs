//! Smart contract SDK for the Kalvora Network.
//!
//! Write contracts in Rust, compile to `wasm32-unknown-unknown`, and deploy
//! to Kalvora validators. `kal-sc` wraps the native host ABI exposed by
//! validators in a safe, idiomatic Rust API, handling serialization, memory
//! management, and the different execution contexts (direct calls,
//! `delegatecall`, derived wallets, etc.).
//!
//! # Quick start
//!
//! ```ignore
//! use kal_sc::{emit, wasmedge_bindgen};
//!
//! #[wasmedge_bindgen]
//! pub fn hello(name: String) {
//!     emit(&format!("hello, {}", name));
//! }
//! ```
//!
//! # Modules
//!
//! - [`transfer`] — move tokens: `send`, `send_multi`, `send_all`, `deposit`, `burn`, `mint`
//! - [`state`] — persist data: `save`, `load`, `clear`, `exists`
//! - [`query`] — read network state: balances, token metadata, contract existence
//! - [`call`] — invoke other contracts: `execute` or `delegate`
//! - [`txn`] — submit raw network transactions
//! - [`wallet`] — derive wallets owned by this contract
//! - [`context`] — read the current frame: sender, block time, contract identity
//! - [`crypto`] — hashing primitives: `sha256`, `sha512`, `blake3`, `shake`
//!
//! # Non-WASM targets
//!
//! On targets other than `wasm32`, every host function is stubbed to panic,
//! so the crate compiles (and docs build) everywhere, but contract logic can
//! only be exercised inside a validator.

#![allow(clippy::result_unit_err)]

mod base64;
mod ffi;
mod types;

pub mod call;
pub mod context;
pub mod crypto;
pub mod query;
pub mod state;
pub mod transfer;
pub mod txn;
pub mod wallet;

pub use types::*;

#[allow(unused_imports)]
use wasmedge_bindgen::*;
pub use wasmedge_bindgen_macro::wasmedge_bindgen;

use serde::{de::DeserializeOwned, Serialize};

/// The canonical Kalvora Network contract identifier.
pub const KALVORA_NETWORK_CONTRACT_ID: &str = "KALXvxhUMJERCse4e6b2jeXFkcqqpiUByQQckvPZm4szmF3Ao";

/// The PreGov contract identifier.
pub const PREGOV_CONTRACT_ID: &str = "PREDNmAbR9Juj233WUQ6kqFGf28oe5kVAK5Y3mhGfE6cVBF95";

/// The Kip contract identifier.
pub const KIP_CONTRACT_ID: &str = "K1PoaARovFYFLAqWRoXubXBkJ2dSynjzUyRzsBc3ycbGEn9pK";

/// The Legal contract identifier.
pub const LEGAL_CONTRACT_ID: &str = "LEGWZ2q8Dw1B8Z9r61STBkaFLPMfDYQPQXFEzJzmQEdyw9tg5";

/// The Mint contract identifier.
pub const MINT_CONTRACT_ID: &str = "M1NTxB3N3GeM5CZLHj3bDLtuye15VQxTTJ3omn9g9Ti87o9b3";

/// The Treasury contract identifier.
pub const TREASURY_CONTRACT_ID: &str = "TREF2UL7Ut6JDTcAaGfY32UrQv9aC5MVyyDjNbFKjhAq51cet";

/// The Tech contract identifier.
pub const TECH_CONTRACT_ID: &str = "TECfnGgEykEs1G57P43ABBb6i73LgwAbHLPf46z1kGyY9A7PY";

/// The Marketing contract identifier.
pub const MARKETING_CONTRACT_ID: &str = "MARYCY87a7ZdeYERoCJbwFv8r6Sm69zjKEspEEo91kcDGH3ce";

/// The BridgeTokens contract identifier.
pub const BRIDGE_TOKENS_CONTRACT_ID: &str = "BRT3zKHfF8RgqbaYGTrv58j3LpNUr5EAJBVn63sBfpTE7AfFk";

/// The BridgeGuardians contract identifier.
pub const BRIDGE_GUARDIANS_CONTRACT_ID: &str =
    "BRGhGqDbqLMqYNRdCSbBVujfQurhW5x3AkQQPnUUCe5Ctb5ua";

/// Emit a log message from the contract. Visible in transaction results.
pub fn emit(message: &str) {
    let ptr = message.as_bytes().as_ptr();
    let len = message.len() as i32;
    unsafe { ffi::emit(ptr, len) };
}

/// Emit a block-level log message from the contract.
pub fn emit_block(message: &str) {
    let ptr = message.as_bytes().as_ptr();
    let len = message.len() as i32;
    unsafe { ffi::emit_block(ptr, len) };
}

/// Encode `value` as a postcard-serialized, base64-encoded string.
///
/// Useful for passing structured data out of a contract via `emit`, or for
/// preparing payloads to be consumed later by [`decode`].
pub fn encode<T: Serialize>(value: &T) -> Result<String, ()> {
    let bytes = postcard::to_allocvec(value).map_err(|_| ())?;
    Ok(base64::encode(&bytes))
}

/// Decode a postcard-serialized, base64-encoded string back into `T`.
///
/// Useful for decoding structured parameters passed into a contract function
/// as a single encoded string.
pub fn decode<T: DeserializeOwned>(encoded: &str) -> Result<T, ()> {
    let bytes = base64::decode(encoded)?;
    postcard::from_bytes(&bytes).map_err(|_| ())
}
