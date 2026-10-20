use crate::ffi;

fn ffi_query_one(
    arg: &str,
    call: unsafe extern "C" fn(*const u8, i32, *const u8) -> i32,
) -> String {
    let mut buffer = Vec::with_capacity(0);
    let target = buffer.as_mut_ptr();
    let len = unsafe { call(arg.as_ptr(), arg.len() as i32, target) } as usize;
    unsafe { buffer.set_len(len) };
    String::from_utf8(buffer).unwrap()
}

fn ffi_query_two(
    a: &str,
    b: &str,
    call: unsafe extern "C" fn(*const u8, i32, *const u8, i32, *const u8) -> i32,
) -> String {
    let mut buffer = Vec::with_capacity(0);
    let target = buffer.as_mut_ptr();
    let len = unsafe {
        call(
            a.as_ptr(),
            a.len() as i32,
            b.as_ptr(),
            b.len() as i32,
            target,
        )
    } as usize;
    unsafe { buffer.set_len(len) };
    String::from_utf8(buffer).unwrap()
}

/// Check a wallet's balance for a specific token.
pub fn wallet_balance(token: &str, wallet: &str) -> String {
    ffi_query_two(token, wallet, ffi::wallet_balance)
}

/// List all tokens held by a wallet.
pub fn wallet_tokens(wallet: &str) -> Vec<String> {
    let res = ffi_query_one(wallet, ffi::wallet_tokens);
    if res.is_empty() {
        return vec![];
    }
    res.split(',').map(|s| s.to_string()).collect()
}

/// Check if a token contract exists on the network.
pub fn contract_exists(contract_id: &str) -> bool {
    ffi_query_one(contract_id, ffi::contract_exists) == "true"
}

/// Check if a smart contract instance exists on the network.
pub fn smart_contract_exists(contract: &str, instance: Option<&str>) -> bool {
    let inst = instance.unwrap_or("1");
    ffi_query_two(contract, inst, ffi::smart_contract_exists) == "true"
}

/// Check if a wallet address exists on the network.
pub fn wallet_exists(wallet: &str) -> bool {
    ffi_query_one(wallet, ffi::wallet_exists) == "true"
}

/// Get the denomination of a token contract.
pub fn contract_denomination(contract_id: &str) -> String {
    ffi_query_one(contract_id, ffi::contract_denomination)
}

/// Get the display name of a token contract.
pub fn token_name(contract_id: &str) -> String {
    ffi_query_one(contract_id, ffi::contract_name)
}

/// Get the symbol of a token contract.
pub fn token_symbol(contract_id: &str) -> String {
    ffi_query_one(contract_id, ffi::contract_symbol)
}

/// Validator status for a main wallet, as returned by [`validator_info`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ValidatorInfo {
    /// The wallet belongs to a currently registered validator.
    pub is_validator: bool,
    /// The validator is online and has sent a heartbeat within the last 500 blocks.
    pub online: bool,
    /// Always `false` until slashing is tracked on the network.
    pub slashed: bool,
    /// The validator has unbonded and is inside the 7-day unbonding period.
    pub unbonding: bool,
}

/// Look up a validator by its main wallet address (Base58).
///
/// This is the wallet derived from the validator's original public key, not
/// its generated operational key. Unknown wallets and invalid Base58 return
/// every field as `false`.
pub fn validator_info(wallet: &str) -> ValidatorInfo {
    let res = ffi_query_one(wallet, ffi::validator_info);
    let flags: Vec<bool> = res.split(',').map(|s| s == "true").collect();
    let flag = |i: usize| flags.get(i).copied().unwrap_or(false);
    ValidatorInfo {
        is_validator: flag(0),
        online: flag(1),
        slashed: flag(2),
        unbonding: flag(3),
    }
}

/// Resolve a smart contract's wallet address from its name and instance.
///
/// `instance` defaults to `"1"` if `None`. Returns an empty string if the
/// contract doesn't exist.
pub fn contract_wallet(contract: &str, instance: Option<&str>) -> String {
    let inst = instance.unwrap_or("1");
    let sc_key = format!("{contract}_{inst}");
    ffi_query_one(&sc_key, ffi::contract_wallet)
}

/// Get the circulating supply of a token contract.
pub fn circulating_supply(contract_id: &str) -> String {
    ffi_query_one(contract_id, ffi::circulating_supply)
}

/// Get the called contract's balance for a specific token.
pub fn sc_balance(token: &str) -> String {
    ffi_query_one(token, ffi::smart_contract_balance)
}

/// Get the current contract's balance in a delegatecall chain.
pub fn current_sc_balance(token: &str) -> String {
    ffi_query_one(token, ffi::current_smart_contract_balance)
}

/// Check if a wallet is compliant for a token contract.
pub fn compliance(contract_id: &str, wallet: &str) -> bool {
    ffi_query_two(contract_id, wallet, ffi::compliance) == "true"
}

/// Get compliance levels for a wallet on a token contract.
pub fn compliance_levels(contract_id: &str, wallet: &str) -> Vec<u32> {
    let res = ffi_query_two(contract_id, wallet, ffi::compliance_levels);
    if res == "0" {
        return vec![];
    }
    res.split(',')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

/// Get authorized currency equivalent data for a contract.
pub fn ace_data(contract_id: &str) -> (bool, String) {
    let res = ffi_query_one(contract_id, ffi::get_ace_data);
    let parts: Vec<&str> = res.splitn(2, ',').collect();
    let authorized = parts[0] == "true";
    let amount = parts.get(1).unwrap_or(&"").to_string();
    (authorized, amount)
}
