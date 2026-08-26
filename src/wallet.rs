use crate::ffi;

fn ffi_derive(seed: &str, call: unsafe extern "C" fn(*const u8, i32, *const u8) -> i32) -> String {
    let mut buffer = Vec::with_capacity(0);
    let target = buffer.as_mut_ptr();
    let len = unsafe { call(seed.as_ptr(), seed.len() as i32, target) } as usize;
    unsafe { buffer.set_len(len) };
    String::from_utf8(buffer).unwrap()
}

/// Derive a wallet from this contract using a seed.
/// Returns the derived wallet address. Creates it if it doesn't already exist.
pub fn derive(seed: &str) -> String {
    ffi_derive(seed, ffi::derive_wallet)
}

/// Derive a wallet from the current contract in a delegatecall chain.
/// Returns the derived wallet address. Creates it if it doesn't already exist.
pub fn derive_current(seed: &str) -> String {
    ffi_derive(seed, ffi::derive_wallet_current)
}

/// Derive a wallet from a delegated contract.
/// Returns the derived wallet address. Creates it if it doesn't already exist.
/// `instance` defaults to `"1"` if not provided.
pub fn derive_delegate(seed: &str, contract: &str, instance: Option<&str>) -> String {
    let inst = instance.unwrap_or("1");
    let mut buffer = Vec::with_capacity(0);
    let target = buffer.as_mut_ptr();
    let len = unsafe {
        ffi::derive_wallet_delegate(
            seed.as_ptr(),
            seed.len() as i32,
            contract.as_ptr(),
            contract.len() as i32,
            inst.as_ptr(),
            inst.len() as i32,
            target,
        )
    } as usize;
    unsafe { buffer.set_len(len) };
    String::from_utf8(buffer).unwrap()
}

/// Build the smart-contract auth-key string for a given contract and instance.
///
/// The network identifies a smart contract as a restricted-key holder using
/// the string `"sc_<contract_name>_<instance>"`. Use this helper when
/// populating `PublicKey::smart_contract_auth` on a `RestrictedKey` — it's
/// purely a client-side string-build, no host call is made.
///
/// `instance` defaults to `"1"` if not provided.
///
/// ```ignore
/// use kal_sc::wallet;
/// let s = wallet::sc_auth("sc_test_proxy", Some("1"));
/// assert_eq!(s, "sc_sc_test_proxy_1");
/// ```
pub fn sc_auth(contract: &str, instance: Option<&str>) -> String {
    let inst = instance.unwrap_or("1");
    format!("sc_{contract}_{inst}")
}
