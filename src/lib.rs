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
pub const KALVORA_NETWORK_CONTRACT_ID: &str = "KALVxuuPAwkBPQKBDAjnMzJcbbk7W6GPhXuNt9XmKZnxdLecM";

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
