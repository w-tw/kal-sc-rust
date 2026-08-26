use crate::base64;
use crate::ffi;
use postcard;
use serde::{de::DeserializeOwned, Serialize};

/// Identifies which contract's state to operate on.
#[derive(Debug, Clone)]
pub enum Target {
    /// This contract's own state.
    This,
    /// A delegated contract's state. `instance` defaults to `"1"` if `None`.
    Delegate {
        contract: String,
        instance: Option<String>,
    },
}

fn encode<T: Serialize>(value: &T) -> String {
    let bytes = postcard::to_allocvec(value).unwrap();
    base64::encode(&bytes)
}

fn decode<T: DeserializeOwned>(b64: String) -> Result<T, ()> {
    let bytes = base64::decode(&b64)?;
    postcard::from_bytes(&bytes).map_err(|_| ())
}

fn delegate_key(contract: &str, instance: &Option<String>) -> String {
    format!("{}_{}", contract, instance.as_deref().unwrap_or("1"))
}

fn ffi_store(key: &str, value: &str) -> bool {
    let key_pointer = key.as_bytes().as_ptr();
    let key_length = key.len() as i32;
    let value_pointer = value.as_bytes().as_ptr();
    let value_length = value.len() as i32;
    unsafe { ffi::store_state(key_pointer, key_length, value_pointer, value_length) != 0 }
}

fn ffi_retrieve(key: &str) -> String {
    let key_pointer = key.as_bytes().as_ptr();
    let key_length = key.len() as i32;
    let mut buffer = Vec::with_capacity(0);
    let target_pointer = buffer.as_mut_ptr();
    let result_len =
        unsafe { ffi::retrieve_state(key_pointer, key_length, target_pointer) } as usize;
    unsafe { buffer.set_len(result_len) };
    String::from_utf8(buffer).unwrap()
}

fn ffi_clear(key: &str) {
    let key_pointer = key.as_bytes().as_ptr();
    let key_length = key.len() as i32;
    unsafe { ffi::clear_state(key_pointer, key_length) };
}

fn ffi_store_delegate(key: &str, value: &str, full_contract: &str) -> bool {
    let key_pointer = key.as_bytes().as_ptr();
    let key_length = key.len() as i32;
    let value_pointer = value.as_bytes().as_ptr();
    let value_length = value.len() as i32;
    let contract_pointer = full_contract.as_bytes().as_ptr();
    let contract_length = full_contract.len() as i32;
    unsafe {
        ffi::delegate_store_state(
            key_pointer,
            key_length,
            value_pointer,
            value_length,
            contract_pointer,
            contract_length,
        ) != 0
    }
}

fn ffi_retrieve_delegate(key: &str, full_contract: &str) -> String {
    let key_pointer = key.as_bytes().as_ptr();
    let key_length = key.len() as i32;
    let contract_pointer = full_contract.as_bytes().as_ptr();
    let contract_length = full_contract.len() as i32;
    let mut buffer = Vec::with_capacity(0);
    let target_pointer = buffer.as_mut_ptr();
    let result_len = unsafe {
        ffi::delegate_retrieve_state(
            key_pointer,
            key_length,
            contract_pointer,
            contract_length,
            target_pointer,
        )
    } as usize;
    unsafe { buffer.set_len(result_len) };
    String::from_utf8(buffer).unwrap()
}

fn ffi_clear_delegate(key: &str, full_contract: &str) {
    let key_pointer = key.as_bytes().as_ptr();
    let key_length = key.len() as i32;
    let contract_pointer = full_contract.as_bytes().as_ptr();
    let contract_length = full_contract.len() as i32;
    unsafe {
        ffi::delegate_clear_state(key_pointer, key_length, contract_pointer, contract_length)
    };
}

// Public functions:
/// Serialize `value` with postcard+base64 and store it under `key`.
pub fn save<T: Serialize>(key: &str, value: &T, target: &Target) -> bool {
    let encoded = encode(value);
    match target {
        Target::This => ffi_store(key, &encoded),
        Target::Delegate { contract, instance } => {
            ffi_store_delegate(key, &encoded, &delegate_key(contract, instance))
        }
    }
}

/// Load and deserialize the value stored under `key`.
/// Returns `Err(())` if the key does not exist or cannot be deserialized.
pub fn load<T: DeserializeOwned>(key: &str, target: &Target) -> Result<T, ()> {
    let raw = match target {
        Target::This => ffi_retrieve(key),
        Target::Delegate { contract, instance } => {
            ffi_retrieve_delegate(key, &delegate_key(contract, instance))
        }
    };
    decode(raw)
}

/// Store a raw string value under `key`.
pub fn save_string(key: &str, value: &str, target: &Target) -> bool {
    match target {
        Target::This => ffi_store(key, value),
        Target::Delegate { contract, instance } => {
            ffi_store_delegate(key, value, &delegate_key(contract, instance))
        }
    }
}

/// Load the raw string value stored under `key`.
/// Returns `None` if the key does not exist.
pub fn load_string(key: &str, target: &Target) -> Option<String> {
    let result = match target {
        Target::This => ffi_retrieve(key),
        Target::Delegate { contract, instance } => {
            ffi_retrieve_delegate(key, &delegate_key(contract, instance))
        }
    };
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

/// Check whether a value is stored under `key`.
pub fn exists(key: &str, target: &Target) -> bool {
    let key_pointer = key.as_bytes().as_ptr();
    let key_length = key.len() as i32;
    match target {
        Target::This => unsafe { ffi::state_exists(key_pointer, key_length) != 0 },
        Target::Delegate { contract, instance } => {
            let full_contract = delegate_key(contract, instance);
            let contract_pointer = full_contract.as_bytes().as_ptr();
            let contract_length = full_contract.len() as i32;
            unsafe {
                ffi::delegate_state_exists(
                    key_pointer,
                    key_length,
                    contract_pointer,
                    contract_length,
                ) != 0
            }
        }
    }
}

/// Remove the value stored under `key`.
pub fn clear(key: &str, target: &Target) {
    match target {
        Target::This => ffi_clear(key),
        Target::Delegate { contract, instance } => {
            ffi_clear_delegate(key, &delegate_key(contract, instance))
        }
    }
}

fn parse_state_pairs(raw: String) -> Vec<(String, String)> {
    let lines: Vec<&str> = raw.split('\n').collect();
    let mut pairs: Vec<(String, String)> = Vec::new();
    for i in (0..lines.len()).step_by(2) {
        if i + 1 < lines.len() {
            pairs.push((lines[i].to_string(), lines[i + 1].to_string()));
        }
    }
    pairs
}

fn ffi_list_self() -> String {
    let mut buffer = Vec::with_capacity(0);
    let target_pointer = buffer.as_mut_ptr();
    let result_len = unsafe { ffi::current_get_all_states(target_pointer) } as usize;
    unsafe { buffer.set_len(result_len) };
    String::from_utf8(buffer).unwrap()
}

fn ffi_list_self_prefix(prefix: &str) -> String {
    let prefix_pointer = prefix.as_bytes().as_ptr();
    let prefix_length = prefix.len() as i32;
    let mut buffer = Vec::with_capacity(0);
    let target_pointer = buffer.as_mut_ptr();
    let result_len = unsafe {
        ffi::current_get_all_states_by_prefix(prefix_pointer, prefix_length, target_pointer)
    } as usize;
    unsafe { buffer.set_len(result_len) };
    String::from_utf8(buffer).unwrap()
}

/// List all key-value pairs in a contract's state.
/// Returns a vec of `(key, value)` string pairs.
pub fn list(target: &Target) -> Vec<(String, String)> {
    let raw = match target {
        Target::This => ffi_list_self(),
        Target::Delegate { contract, instance } => {
            let full_contract = delegate_key(contract, instance);
            let contract_pointer = full_contract.as_bytes().as_ptr();
            let contract_length = full_contract.len() as i32;
            let mut buffer = Vec::with_capacity(0);
            let target_pointer = buffer.as_mut_ptr();
            let result_len =
                unsafe { ffi::get_all_states(contract_pointer, contract_length, target_pointer) }
                    as usize;
            unsafe { buffer.set_len(result_len) };
            String::from_utf8(buffer).unwrap()
        }
    };
    parse_state_pairs(raw)
}

/// List all key-value pairs in a contract's state whose keys start with `prefix`.
/// Returns a vec of `(key, value)` string pairs.
pub fn list_prefix(prefix: &str, target: &Target) -> Vec<(String, String)> {
    let raw = match target {
        Target::This => ffi_list_self_prefix(prefix),
        Target::Delegate { contract, instance } => {
            let full_contract = delegate_key(contract, instance);
            let contract_pointer = full_contract.as_bytes().as_ptr();
            let contract_length = full_contract.len() as i32;
            let prefix_pointer = prefix.as_bytes().as_ptr();
            let prefix_length = prefix.len() as i32;
            let mut buffer = Vec::with_capacity(0);
            let target_pointer = buffer.as_mut_ptr();
            let result_len = unsafe {
                ffi::get_all_states_by_prefix(
                    contract_pointer,
                    contract_length,
                    prefix_pointer,
                    prefix_length,
                    target_pointer,
                )
            } as usize;
            unsafe { buffer.set_len(result_len) };
            String::from_utf8(buffer).unwrap()
        }
    };
    parse_state_pairs(raw)
}
