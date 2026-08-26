use crate::ffi;

fn ffi_call(
    contract: &str,
    instance: &str,
    function: &str,
    params: &[String],
    call: unsafe extern "C" fn(
        *const u8,
        i32,
        *const u8,
        i32,
        *const u8,
        i32,
        *const u8,
        i32,
        *const u8,
    ) -> i32,
) -> Vec<String> {
    let joined = params.join("##");
    let mut buffer = Vec::with_capacity(0);
    let target = buffer.as_mut_ptr();
    let len = unsafe {
        call(
            contract.as_ptr(),
            contract.len() as i32,
            instance.as_ptr(),
            instance.len() as i32,
            function.as_ptr(),
            function.len() as i32,
            joined.as_ptr(),
            joined.len() as i32,
            target,
        )
    } as usize;
    unsafe { buffer.set_len(len) };
    let res = String::from_utf8(buffer).unwrap();
    res.split("[res]")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_end_matches("[end]").to_string())
        .collect()
}

/// Call a function on another smart contract.
/// Executes in the context of the *target* contract (its own state and wallet).
/// Returns the result strings emitted by the target function.
pub fn execute(contract: &str, instance: &str, function: &str, params: &[String]) -> Vec<String> {
    ffi_call(contract, instance, function, params, ffi::call)
}

/// Delegatecall a function on another smart contract.
/// Executes in the context of the *calling* contract (this contract's state and wallet).
/// Returns the result strings emitted by the target function.
pub fn delegate(contract: &str, instance: &str, function: &str, params: &[String]) -> Vec<String> {
    ffi_call(contract, instance, function, params, ffi::delegatecall)
}
