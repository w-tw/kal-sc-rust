use crate::ffi;
use crate::query;
use crate::wallet;

/// Identifies which wallet the operation acts through.
#[derive(Debug, Clone)]
pub enum Sender {
    /// The wallet of whoever invoked the contract (the transaction signer).
    Caller,
    /// This smart contract's own wallet.
    Sc,
    /// The currently-executing contract in a delegatecall chain.
    Current,
    /// A different smart contract's wallet. `instance` defaults to `"1"` if `None`.
    Delegate {
        contract: String,
        instance: Option<String>,
    },
    /// A derived wallet owned by this smart contract, identified by its seed.
    Derived { seed: String },
    /// A derived wallet owned by the currently-executing contract in a
    /// delegatecall chain, identified by its seed.
    DerivedCurrent { seed: String },
    /// A derived wallet owned by a delegated smart contract. `instance` defaults to `"1"` if `None`.
    DerivedDelegate {
        seed: String,
        contract: String,
        instance: Option<String>,
    },
}

fn result_bool(f: impl FnOnce(*const u8) -> i32) -> bool {
    let mut buf = Vec::with_capacity(0);
    let len = f(buf.as_mut_ptr()) as usize;
    unsafe { buf.set_len(len) };
    matches!(String::from_utf8(buf).as_deref(), Ok("OK"))
}

fn result_string(f: impl FnOnce(*const u8) -> i32) -> String {
    let mut buf = Vec::with_capacity(0);
    let len = f(buf.as_mut_ptr()) as usize;
    unsafe { buf.set_len(len) };
    String::from_utf8(buf).unwrap()
}

/// Send `amount` of `token` to `to`.
pub fn send(token: &str, amount: &str, to: &str, sender: &Sender) -> bool {
    let t = token.as_ptr();
    let tl = token.len() as i32;
    let a = amount.as_ptr();
    let al = amount.len() as i32;
    let d = to.as_ptr();
    let dl = to.len() as i32;

    match sender {
        Sender::Caller => result_bool(|tp| unsafe { ffi::transfer(t, tl, a, al, d, dl, tp) }),
        Sender::Sc => result_bool(|tp| unsafe { ffi::send(t, tl, a, al, d, dl, tp) }),
        Sender::Current => result_bool(|tp| unsafe { ffi::current_send(t, tl, a, al, d, dl, tp) }),
        Sender::Delegate { contract, instance } => {
            let w = query::contract_wallet(contract, instance.as_deref());
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_bool(|tp| unsafe { ffi::delegate_send(t, tl, a, al, d, dl, wp, wl, tp) })
        }
        Sender::Derived { seed } => {
            let w = wallet::derive(seed);
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_bool(|tp| unsafe { ffi::derived_send(t, tl, a, al, d, dl, wp, wl, tp) })
        }
        Sender::DerivedCurrent { seed } => {
            let w = wallet::derive_current(seed);
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_bool(|tp| unsafe { ffi::derived_current_send(t, tl, a, al, d, dl, wp, wl, tp) })
        }
        Sender::DerivedDelegate {
            seed,
            contract,
            instance,
        } => {
            let w = wallet::derive_delegate(seed, contract, instance.as_deref());
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            let c = contract.as_ptr();
            let cl = contract.len() as i32;
            let inst = instance.as_deref().unwrap_or("1");
            let ip = inst.as_ptr();
            let il = inst.len() as i32;
            result_bool(|tp| unsafe {
                ffi::derived_delegate_send(t, tl, a, al, d, dl, wp, wl, c, cl, ip, il, tp)
            })
        }
    }
}

/// Send `token` to multiple recipients in one operation.
/// `amounts` and `to` must be the same length.
/// `input_amount` is the total debited from the sender.
pub fn send_multi(
    token: &str,
    input_amount: &str,
    amounts: &[String],
    to: &[String],
    sender: &Sender,
) -> bool {
    let t = token.as_ptr();
    let tl = token.len() as i32;
    let ia = input_amount.as_ptr();
    let ial = input_amount.len() as i32;
    let amounts_joined = amounts.join(",");
    let aj = amounts_joined.as_ptr();
    let ajl = amounts_joined.len() as i32;
    let addrs_joined = to.join(",");
    let dj = addrs_joined.as_ptr();
    let djl = addrs_joined.len() as i32;

    match sender {
        Sender::Caller => {
            result_bool(|tp| unsafe { ffi::transfer_multi(t, tl, ia, ial, aj, ajl, dj, djl, tp) })
        }
        Sender::Sc => {
            result_bool(|tp| unsafe { ffi::send_multi(t, tl, ia, ial, aj, ajl, dj, djl, tp) })
        }
        Sender::Current => result_bool(|tp| unsafe {
            ffi::current_send_multi(t, tl, ia, ial, aj, ajl, dj, djl, tp)
        }),
        Sender::Delegate { contract, instance } => {
            let w = query::contract_wallet(contract, instance.as_deref());
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_bool(|tp| unsafe {
                ffi::delegate_send_multi(t, tl, ia, ial, aj, ajl, dj, djl, wp, wl, tp)
            })
        }
        Sender::Derived { seed } => {
            let w = wallet::derive(seed);
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_bool(|tp| unsafe {
                ffi::derived_send_multi(t, tl, ia, ial, aj, ajl, dj, djl, wp, wl, tp)
            })
        }
        Sender::DerivedCurrent { seed } => {
            let w = wallet::derive_current(seed);
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_bool(|tp| unsafe {
                ffi::derived_current_send_multi(t, tl, ia, ial, aj, ajl, dj, djl, wp, wl, tp)
            })
        }
        Sender::DerivedDelegate {
            seed,
            contract,
            instance,
        } => {
            let w = wallet::derive_delegate(seed, contract, instance.as_deref());
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            let c = contract.as_ptr();
            let cl = contract.len() as i32;
            let inst = instance.as_deref().unwrap_or("1");
            let ip = inst.as_ptr();
            let il = inst.len() as i32;
            result_bool(|tp| unsafe {
                ffi::derived_delegate_send_multi(
                    t, tl, ia, ial, aj, ajl, dj, djl, wp, wl, c, cl, ip, il, tp,
                )
            })
        }
    }
}

/// Send the entire balance of all tokens to `to`.
pub fn send_all(to: &str, sender: &Sender) -> String {
    let d = to.as_ptr();
    let dl = to.len() as i32;

    match sender {
        Sender::Caller => result_string(|tp| unsafe { ffi::transfer_all(d, dl, tp) }),
        Sender::Sc => result_string(|tp| unsafe { ffi::send_all(d, dl, tp) }),
        Sender::Current => result_string(|tp| unsafe { ffi::current_send_all(d, dl, tp) }),
        Sender::Delegate { contract, instance } => {
            let w = query::contract_wallet(contract, instance.as_deref());
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_string(|tp| unsafe { ffi::delegate_send_all(d, dl, wp, wl, tp) })
        }
        Sender::Derived { seed } => {
            let w = wallet::derive(seed);
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_string(|tp| unsafe { ffi::derived_send_all(d, dl, wp, wl, tp) })
        }
        Sender::DerivedCurrent { seed } => {
            let w = wallet::derive_current(seed);
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_string(|tp| unsafe { ffi::derived_current_send_all(d, dl, wp, wl, tp) })
        }
        Sender::DerivedDelegate {
            seed,
            contract,
            instance,
        } => {
            let w = wallet::derive_delegate(seed, contract, instance.as_deref());
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            let c = contract.as_ptr();
            let cl = contract.len() as i32;
            let inst = instance.as_deref().unwrap_or("1");
            let ip = inst.as_ptr();
            let il = inst.len() as i32;
            result_string(|tp| unsafe {
                ffi::derived_delegate_send_all(d, dl, wp, wl, c, cl, ip, il, tp)
            })
        }
    }
}

/// Deposit the caller's `amount` of `token` into a contract wallet.
///
/// `Sender::Caller` is not supported and returns `false`.
pub fn deposit(token: &str, amount: &str, into: &Sender) -> bool {
    let t = token.as_ptr();
    let tl = token.len() as i32;
    let a = amount.as_ptr();
    let al = amount.len() as i32;

    match into {
        Sender::Caller => false,
        Sender::Sc => result_bool(|tp| unsafe { ffi::hold(t, tl, a, al, tp) }),
        Sender::Current => result_bool(|tp| unsafe { ffi::current_hold(t, tl, a, al, tp) }),
        Sender::Delegate { contract, instance } => {
            let w = query::contract_wallet(contract, instance.as_deref());
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_bool(|tp| unsafe { ffi::delegate_hold(t, tl, a, al, wp, wl, tp) })
        }
        Sender::Derived { seed } => {
            let w = wallet::derive(seed);
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_bool(|tp| unsafe { ffi::derived_hold(t, tl, a, al, wp, wl, tp) })
        }
        Sender::DerivedCurrent { seed } => {
            let w = wallet::derive_current(seed);
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_bool(|tp| unsafe { ffi::derived_current_hold(t, tl, a, al, wp, wl, tp) })
        }
        Sender::DerivedDelegate {
            seed,
            contract,
            instance,
        } => {
            let w = wallet::derive_delegate(seed, contract, instance.as_deref());
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            let c = contract.as_ptr();
            let cl = contract.len() as i32;
            let inst = instance.as_deref().unwrap_or("1");
            let ip = inst.as_ptr();
            let il = inst.len() as i32;
            result_bool(|tp| unsafe {
                ffi::derived_delegate_hold(t, tl, a, al, wp, wl, c, cl, ip, il, tp)
            })
        }
    }
}

/// Burn `amount` of `token` from the sender's wallet.
pub fn burn(token: &str, amount: &str, sender: &Sender) -> bool {
    send(token, amount, ":fire:", sender)
}

/// Mint `amount` of `token` to `to`.
///
/// Only supports `Sender::Sc`, `Current`, and `Delegate`.
/// `Caller`, `Derived`, `DerivedCurrent`, and `DerivedDelegate` return `false`.
pub fn mint(token: &str, amount: &str, to: &str, sender: &Sender) -> bool {
    let t = token.as_ptr();
    let tl = token.len() as i32;
    let a = amount.as_ptr();
    let al = amount.len() as i32;
    let d = to.as_ptr();
    let dl = to.len() as i32;

    match sender {
        Sender::Sc => result_bool(|tp| unsafe { ffi::mint(t, tl, a, al, d, dl, tp) }),
        Sender::Current => result_bool(|tp| unsafe { ffi::current_mint(t, tl, a, al, d, dl, tp) }),
        Sender::Delegate { contract, instance } => {
            let w = query::contract_wallet(contract, instance.as_deref());
            let wp = w.as_ptr();
            let wl = w.len() as i32;
            result_bool(|tp| unsafe { ffi::delegate_mint(t, tl, a, al, d, dl, wp, wl, tp) })
        }
        _ => false,
    }
}
