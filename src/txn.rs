use crate::base64;
use crate::ffi;
use crate::types::*;
use serde::Serialize;

fn serialize<T: Serialize>(txn: &T) -> String {
    let bytes = postcard::to_allocvec(txn).unwrap();
    base64::encode(&bytes)
}

pub(crate) fn send_to_network(network_txn: NetworkTXN) -> bool {
    let serialized = serialize(&network_txn);

    let mut buffer = Vec::with_capacity(0);
    let result_len = unsafe {
        ffi::submit_txn(
            serialized.as_bytes().as_ptr(),
            serialized.len() as i32,
            buffer.as_mut_ptr(),
        )
    } as usize;

    unsafe { buffer.set_len(result_len) };

    match String::from_utf8(buffer) {
        Ok(res) => res.trim() == "OK",
        Err(_) => false,
    }
}

pub fn contract_update(txn: &ContractUpdateTXN, sender: TxnSender) -> bool {
    send_to_network(NetworkTXN {
        txn_type: TransactionType::UpdateContractType,
        payload: serialize(txn),
        sender,
    })
}

pub fn contract(txn: &InstrumentContractTXN, sender: TxnSender) -> bool {
    send_to_network(NetworkTXN {
        txn_type: TransactionType::ContractTxnType,
        payload: serialize(txn),
        sender,
    })
}

pub fn coin(txn: &CoinTXN, sender: TxnSender) -> bool {
    send_to_network(NetworkTXN {
        txn_type: TransactionType::CoinType,
        payload: serialize(txn),
        sender,
    })
}

pub fn mint(txn: &MintTXN, sender: TxnSender) -> bool {
    send_to_network(NetworkTXN {
        txn_type: TransactionType::MintType,
        payload: serialize(txn),
        sender,
    })
}
