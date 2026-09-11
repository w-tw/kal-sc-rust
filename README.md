# kal-sc

Smart contract SDK for the Kalvora Network. Write contracts in Rust, compile
to WebAssembly, and deploy to Kalvora validators.

`kal-sc` wraps the native host ABI exposed by Kalvora validators in a safe,
idiomatic Rust API. It handles serialization, memory management, and the
different execution contexts (direct calls, `delegatecall`, derived wallets,
etc.) so you can focus on contract logic.

Kalvora Network contract ID:

```text
KALXvxhUMJERCse4e6b2jeXFkcqqpiUByQQckvPZm4szmF3Ao
```

The same value is exported as `kal_sc::KALVORA_NETWORK_CONTRACT_ID`. IDs for
the other core network contracts are exported alongside it:

| Constant                      | Contract        |
|-------------------------------|-----------------|
| `KALVORA_NETWORK_CONTRACT_ID` | KAL             |
| `PREGOV_CONTRACT_ID`          | PreGov          |
| `KIP_CONTRACT_ID`             | Kip             |
| `LEGAL_CONTRACT_ID`           | Legal           |
| `MINT_CONTRACT_ID`            | Mint            |
| `TREASURY_CONTRACT_ID`        | Treasury        |
| `TECH_CONTRACT_ID`            | Tech            |
| `MARKETING_CONTRACT_ID`       | Marketing       |
| `BRIDGE_TOKENS_CONTRACT_ID`   | BridgeTokens    |
| `BRIDGE_GUARDIANS_CONTRACT_ID`| BridgeGuardians |

## Installation

```toml
[dependencies]
kal-sc = "0.1"
```

You will also need the WASM target:

```bash
rustup target add wasm32-unknown-unknown
```

## Hello contract

```rust
use kal_sc::{emit, wasmedge_bindgen};

#[wasmedge_bindgen]
pub fn init() {
    emit("contract initialized");
}

#[wasmedge_bindgen]
pub fn hello(name: String) {
    emit(&format!("hello, {}", name));
}
```

Build with:

```bash
cargo build --target wasm32-unknown-unknown --release
```

The resulting `.wasm` in `target/wasm32-unknown-unknown/release/` is what you
deploy to the Kalvora Network.

> **Note:** `kal-sc` only works inside a Kalvora validator. On non-WASM
> targets every host function is stubbed to panic with a clear message, so the
> crate compiles (and docs build) everywhere, but contract logic can only be
> exercised on-chain.

## Module overview

| Module       | Purpose                                                                   |
|--------------|---------------------------------------------------------------------------|
| `transfer`   | Move tokens: `send`, `send_multi`, `send_all`, `deposit`, `burn`, `mint`. |
| `state`      | Persist data: `save`, `load`, `clear`, `exists`, scoped by `Target`.      |
| `query`      | Read network state: balances, token metadata, contract existence, etc.    |
| `call`       | Invoke other contracts: `execute` (context-switch) or `delegate`.         |
| `txn`        | Submit raw network transactions (`CoinTXN`, `MintTXN`, contract create).  |
| `wallet`     | Derive wallets owned by this contract (`derive`, `derive_delegate`, …).   |
| `context`    | Read the current frame: sender, block time, contract identity, etc.      |
| `crypto`     | Hashing primitives: `sha256`, `sha512`, `blake3`, `shake`.                |

Top-level helpers:

- `emit(msg)` / `emit_block(msg)` — log messages from the contract.
- `encode(&value)` / `decode(&s)` — round-trip any `Serialize` type through
  `postcard + base64`, useful for packing structured data into a single
  parameter string.

Full API docs: [docs.rs/kal-sc](https://docs.rs/kal-sc).

## The `Sender` enum

The most important concept in `kal-sc` is the [`transfer::Sender`] enum. It
tells the host *which wallet* an operation should act through. Pick the wrong
variant and your transfer will move funds from the wrong place (or be rejected).

| Variant                      | Wallet used                                                                         |
|------------------------------|-------------------------------------------------------------------------------------|
| `Sender::Caller`             | The user who signed the outer transaction.                                          |
| `Sender::Sc`                 | This contract's wallet (the originally-called contract).                            |
| `Sender::Current`            | The wallet of the currently-executing contract in a `delegatecall` chain.           |
| `Sender::Delegate { ... }`   | A different contract's wallet, resolved from `(name, instance)`.                    |
| `Sender::Derived { seed }`   | A wallet derived from this contract via `wallet::derive`.                           |
| `Sender::DerivedCurrent`     | A wallet derived from the currently-executing contract via `wallet::derive_current`.|
| `Sender::DerivedDelegate`    | A wallet derived from a different contract via `wallet::derive_delegate`.           |

In a contract called directly (no `delegatecall`), `Sc` and `Current` are the
same wallet. They only diverge once one contract delegatecalls into another.

## Example — token operations

```rust
use kal_sc::transfer::{self, Sender};
use kal_sc::{emit, wasmedge_bindgen};

#[wasmedge_bindgen]
pub fn payout(token: String, amount: String, to: String) {
    // Pay out from the caller's wallet (requires their signature).
    let ok = transfer::send(&token, &amount, &to, &Sender::Caller);
    emit(&format!("payout ok={}", ok));
}

#[wasmedge_bindgen]
pub fn fund_self(token: String, amount: String) {
    // Move tokens from the caller into this contract's wallet.
    let ok = transfer::deposit(&token, &amount, &Sender::Sc);
    emit(&format!("fund ok={}", ok));
}
```

## Example — state

```rust
use serde::{Deserialize, Serialize};
use kal_sc::state::{self, Target};
use kal_sc::{emit, wasmedge_bindgen};

#[derive(Serialize, Deserialize, Default)]
struct Profile {
    name: String,
    score: u64,
}

#[wasmedge_bindgen]
pub fn set_profile(name: String, score: String) {
    let p = Profile { name, score: score.parse().unwrap_or(0) };
    state::save("PROFILE", &p, &Target::This);
}

#[wasmedge_bindgen]
pub fn get_profile() {
    let p: Profile = state::load("PROFILE", &Target::This).unwrap_or_default();
    emit(&format!("{}:{}", p.name, p.score));
}
```

## Example — proxy / `delegatecall`

A common pattern: a small **proxy** contract that `delegatecall`s into a larger
**logic** contract, allowing the logic to be swapped without migrating user
state.

```rust
use kal_sc::{call, emit, wasmedge_bindgen};

#[wasmedge_bindgen]
pub fn execute(function: String, parameters: String) {
    let params: Vec<String> = if parameters.is_empty() {
        vec![]
    } else {
        parameters.split(',').map(String::from).collect()
    };
    let results = call::delegate("my_logic", "1", &function, &params);
    for r in results {
        emit(&r);
    }
}
```

In this setup, inside `my_logic`:

- `Sender::Sc` and `Sender::Caller` refer to the proxy / outer caller.
- `Sender::Current` refers to `my_logic`'s own wallet.

## Creating contracts and minting

Token and NFT contracts can be created from inside another contract via
`txn::contract`. To mint into a contract you own, you must register the minting
authority as a `RestrictedKey` at creation time:

```rust
use kal_sc::txn;
use kal_sc::{
    CoinDenomination, InstrumentContractTXN, PublicKey, RestrictedKey, TxnSender,
};

let minter = RestrictedKey {
    public_key: PublicKey::sc("my_logic", Some("1")),
    mint: true,
    update_contract: true,
    key_weight: 1,
    ..Default::default()
};

let new_contract = InstrumentContractTXN {
    contract_id: "$MINE+0001".to_string(),
    contract_version: 1,
    symbol: "MINE".to_string(),
    name: "My Token".to_string(),
    restricted_keys: vec![minter],
    max_supply: Some("1000000000000".to_string()),
    coin_denomination: Some(CoinDenomination {
        denomination_name: "parts".to_string(),
        amount: "1".to_string(),
    }),
    ..Default::default()
};

txn::contract(&new_contract, TxnSender::Current);
```

`PublicKey::sc(name, instance)` populates `smart_contract_auth` with the
string `"sc_<name>_<instance>"`, which is the format the network uses to
identify a smart contract as a restricted-key holder.

## Targets and builds

`kal-sc` is designed for `wasm32-unknown-unknown` only. Host functions are
stubbed to panic when compiled for other targets, so the crate builds on any
target but calling any host-backed function off-chain will panic.

Recommended release build:

```bash
cargo build --target wasm32-unknown-unknown --release
```

For a smaller output, add to the consuming crate's `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

## MSRV

Rust 1.70 or newer.

## Further reading

- Full API reference: [docs.rs/kal-sc](https://docs.rs/kal-sc)
- Source repository: [github.com/w-tw/kal-sc-rust](https://github.com/w-tw/kal-sc-rust)

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or
<https://www.apache.org/licenses/LICENSE-2.0>).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate shall be licensed as Apache-2.0, without any
additional terms or conditions.
