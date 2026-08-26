use crate::ffi;

fn ffi_read(call: unsafe extern "C" fn(*const u8) -> i32) -> String {
    let mut buffer = Vec::with_capacity(0);
    let target_pointer = buffer.as_mut_ptr();
    let result_len = unsafe { call(target_pointer) } as usize;
    unsafe { buffer.set_len(result_len) };
    String::from_utf8(buffer).unwrap()
}

/// The wallet address of the user who signed the transaction.
pub fn sender_address() -> String {
    ffi_read(ffi::wallet_address)
}

/// The public key of the user who signed the transaction.
pub fn sender_public_key() -> String {
    ffi_read(ffi::public_key)
}

/// The hash of the current transaction.
pub fn txn_hash() -> String {
    ffi_read(ffi::txn_hash)
}

/// The timestamp of the latest block (unix seconds).
///
/// Returns `0` if the host returns a malformed value.
pub fn block_time() -> u64 {
    ffi_read(ffi::last_block_time).parse::<u64>().unwrap_or(0)
}

/// This contract's wallet address (the original contract in a delegatecall chain).
pub fn contract_wallet() -> String {
    ffi_read(ffi::smart_contract_wallet)
}

/// The wallet of the currently-executing contract in a delegatecall chain.
pub fn current_wallet() -> String {
    ffi_read(ffi::current_smart_contract_wallet)
}

/// The wallet of the contract that called this contract.
pub fn caller_wallet() -> String {
    ffi_read(ffi::called_smart_contract_wallet)
}

/// The name and instance of the currently-executing contract.
/// Returns `(name, instance)`.
pub fn current_smart_contract() -> (String, String) {
    let combined = ffi_read(ffi::current_smart_contract);
    match combined.rsplit_once('_') {
        Some((name, instance)) => (name.to_string(), instance.to_string()),
        None => (combined, String::new()),
    }
}
