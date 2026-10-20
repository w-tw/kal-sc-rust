use crate::base64;
use crate::ffi;
use crate::types::*;
use serde::Serialize;

// These domain markers precede the new positional Postcard layout. Ten 0xff
// bytes cannot begin a valid legacy u64/string-length varint.
const CONTRACT_V2_PREFIX: &[u8] = b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\xffKAL-CONTRACT-V2";
const CONTRACT_UPDATE_V2_PREFIX: &[u8] = b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\xffKAL-CONTRACT-UPDATE-V2";

fn serialize_native<T: Serialize>(txn: &T, prefix: &[u8]) -> String {
    let mut bytes = prefix.to_vec();
    bytes.extend(postcard::to_allocvec(txn).unwrap());
    base64::encode(&bytes)
}

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
        payload: serialize_native(txn, CONTRACT_UPDATE_V2_PREFIX),
        sender,
    })
}

pub fn contract(txn: &InstrumentContractTXN, sender: TxnSender) -> bool {
    send_to_network(NetworkTXN {
        txn_type: TransactionType::ContractTxnType,
        payload: serialize_native(txn, CONTRACT_V2_PREFIX),
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_wire_numbers_keep_retired_holes() {
        let values = [
            TransactionType::UnknownType, TransactionType::CoinType,
            TransactionType::MintType, TransactionType::ItemMintType,
            TransactionType::ContractTxnType, TransactionType::VoteType,
            TransactionType::ProposalType, TransactionType::SmartContractType,
            TransactionType::SmartContractExecuteType, TransactionType::ExpenseRatioType,
            TransactionType::NftType, TransactionType::UpdateContractType,
            TransactionType::ValidatorRegistrationType, TransactionType::ValidatorHeartbeatType,
            TransactionType::ProposalResultType, TransactionType::DelegatedVotingType,
            TransactionType::RevokeType, TransactionType::ComplianceType,
            TransactionType::SbtBurnType, TransactionType::RequiredVersion,
            TransactionType::SmartContractInstantiateType, TransactionType::AllowanceType,
            TransactionType::ProposalCancelType,
        ];
        for value in values {
            let bytes = postcard::to_allocvec(&value).unwrap();
            assert_eq!(bytes, vec![value as u8]);
            assert_eq!(postcard::from_bytes::<TransactionType>(&bytes).unwrap(), value);
        }
        for number in [17u8, 18, 25] {
            assert!(postcard::from_bytes::<TransactionType>(&[number]).is_err());
        }
    }

    #[test]
    fn human_readable_transaction_names_remain_names() {
        use serde::Deserialize;
        use serde::de::value::{Error, StrDeserializer};
        assert_eq!(TransactionType::deserialize(StrDeserializer::<Error>::new("CoinType")).unwrap(),
                   TransactionType::CoinType);
        assert_eq!(TransactionType::deserialize(StrDeserializer::<Error>::new("ComplianceType")).unwrap(),
                   TransactionType::ComplianceType);
        for name in ["QuashType", "FastQuorumType", "", "25"] {
            assert!(TransactionType::deserialize(StrDeserializer::<Error>::new(name)).is_err());
        }

        struct NamedVariant;
        type Named = (&'static str, u32, &'static str);
        impl serde::Serializer for NamedVariant {
            type Ok = Named;
            type Error = Error;
            type SerializeSeq = serde::ser::Impossible<Named, Error>;
            type SerializeTuple = serde::ser::Impossible<Named, Error>;
            type SerializeTupleStruct = serde::ser::Impossible<Named, Error>;
            type SerializeTupleVariant = serde::ser::Impossible<Named, Error>;
            type SerializeMap = serde::ser::Impossible<Named, Error>;
            type SerializeStruct = serde::ser::Impossible<Named, Error>;
            type SerializeStructVariant = serde::ser::Impossible<Named, Error>;

            fn serialize_unit_variant(self, name: &'static str, index: u32, variant: &'static str) -> Result<Named, Error> {
                Ok((name, index, variant))
            }

            fn serialize_bool(self, _: bool) -> Result<Named, Error> { unreachable!() }
            fn serialize_i8(self, _: i8) -> Result<Named, Error> { unreachable!() }
            fn serialize_i16(self, _: i16) -> Result<Named, Error> { unreachable!() }
            fn serialize_i32(self, _: i32) -> Result<Named, Error> { unreachable!() }
            fn serialize_i64(self, _: i64) -> Result<Named, Error> { unreachable!() }
            fn serialize_u8(self, _: u8) -> Result<Named, Error> { unreachable!() }
            fn serialize_u16(self, _: u16) -> Result<Named, Error> { unreachable!() }
            fn serialize_u32(self, _: u32) -> Result<Named, Error> { unreachable!() }
            fn serialize_u64(self, _: u64) -> Result<Named, Error> { unreachable!() }
            fn serialize_f32(self, _: f32) -> Result<Named, Error> { unreachable!() }
            fn serialize_f64(self, _: f64) -> Result<Named, Error> { unreachable!() }
            fn serialize_char(self, _: char) -> Result<Named, Error> { unreachable!() }
            fn serialize_str(self, _: &str) -> Result<Named, Error> { unreachable!() }
            fn serialize_bytes(self, _: &[u8]) -> Result<Named, Error> { unreachable!() }
            fn serialize_none(self) -> Result<Named, Error> { unreachable!() }
            fn serialize_some<T: ?Sized + Serialize>(self, _: &T) -> Result<Named, Error> { unreachable!() }
            fn serialize_unit(self) -> Result<Named, Error> { unreachable!() }
            fn serialize_unit_struct(self, _: &'static str) -> Result<Named, Error> { unreachable!() }
            fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _: &'static str, _: &T) -> Result<Named, Error> { unreachable!() }
            fn serialize_newtype_variant<T: ?Sized + Serialize>(self, _: &'static str, _: u32, _: &'static str, _: &T) -> Result<Named, Error> { unreachable!() }
            fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Error> { unreachable!() }
            fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Error> { unreachable!() }
            fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeTupleStruct, Error> { unreachable!() }
            fn serialize_tuple_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeTupleVariant, Error> { unreachable!() }
            fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Error> { unreachable!() }
            fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Error> { unreachable!() }
            fn serialize_struct_variant(self, _: &'static str, _: u32, _: &'static str, _: usize) -> Result<Self::SerializeStructVariant, Error> { unreachable!() }
        }
        assert_eq!(TransactionType::CoinType.serialize(NamedVariant).unwrap(),
                   ("TransactionType", 1, "CoinType"));
        assert_eq!(TransactionType::ComplianceType.serialize(NamedVariant).unwrap(),
                   ("TransactionType", 19, "ComplianceType"));
    }

    #[test]
    fn restricted_key_new_layout_has_only_live_grants() {
        let key = RestrictedKey {
            update_contract: true, transfer: true, mint: true, vote: true,
            propose: true, compliance: true, expense_ratio: true, revoke: true,
            key_weight: 5, ..Default::default()
        };
        // Four empty PublicKey fields, retained inert global slot, eight grants,
        // key weight. No scheduling or cancellation slots occur in this layout.
        assert_eq!(postcard::to_allocvec(&key).unwrap(),
                   vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 5]);
    }

    #[test]
    fn contract_update_v2_golden_payload() {
        let txn = ContractUpdateTXN {
            contract_id: "X".to_string(), contract_version: 1, ..Default::default()
        };
        let postcard_bytes = vec![1, b'X', 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(postcard::to_allocvec(&txn).unwrap(), postcard_bytes);
        let mut expected = CONTRACT_UPDATE_V2_PREFIX.to_vec();
        expected.extend(postcard_bytes);
        assert_eq!(base64::decode(&serialize_native(&txn, CONTRACT_UPDATE_V2_PREFIX)).unwrap(), expected);
        assert_ne!(serialize_native(&txn, CONTRACT_UPDATE_V2_PREFIX), serialize(&txn));
    }

    #[test]
    fn instrument_contract_v2_golden_payload() {
        let txn = InstrumentContractTXN {
            contract_version: 1, symbol: "S".to_string(), name: "N".to_string(),
            ..Default::default()
        };
        let postcard_bytes = vec![1, 1, b'S', 1, b'N', 0, 0, 0, 0, 0, 0,
                                  0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(postcard::to_allocvec(&txn).unwrap(), postcard_bytes);
        let mut expected = CONTRACT_V2_PREFIX.to_vec();
        expected.extend(postcard_bytes);
        assert_eq!(base64::decode(&serialize_native(&txn, CONTRACT_V2_PREFIX)).unwrap(), expected);
        assert_ne!(serialize_native(&txn, CONTRACT_V2_PREFIX), serialize(&txn));
    }

    #[test]
    fn mint_payload_remains_unprefixed() {
        let txn = MintTXN {
            contract_id: "X".to_string(), amount: "1".to_string(),
            recipient_address: "Y".to_string(),
        };
        assert_eq!(base64::decode(&serialize(&txn)).unwrap(),
                   vec![1, b'X', 1, b'1', 1, b'Y']);
    }

    #[test]
    fn emit_cross_language_wire_vectors() {
        let grant = RestrictedKey {
            public_key: PublicKey { single: "Key".to_string(), ..Default::default() },
            update_contract: true, transfer: true, mint: true, vote: true,
            propose: true, compliance: true, expense_ratio: true, revoke: true,
            key_weight: 5, ..Default::default()
        };
        let contract = InstrumentContractTXN {
            contract_version: 1, symbol: "S".to_string(), name: "N".to_string(),
            ..Default::default()
        };
        let update = ContractUpdateTXN {
            contract_id: "X".to_string(), contract_version: 1, ..Default::default()
        };
        let mut contract_grants = contract.clone();
        contract_grants.restricted_keys.push(grant.clone());
        let mut update_grants = update.clone();
        update_grants.restricted_keys.push(grant);
        let mint = MintTXN {
            contract_id: "X".to_string(), amount: "1".to_string(),
            recipient_address: "Y".to_string(),
        };
        let coin = CoinTXN { contract_id: "X".to_string(), ..Default::default() };
        for (name, payload) in [
            ("contract_default", serialize_native(&contract, CONTRACT_V2_PREFIX)),
            ("contract_grants", serialize_native(&contract_grants, CONTRACT_V2_PREFIX)),
            ("contract_update_default", serialize_native(&update, CONTRACT_UPDATE_V2_PREFIX)),
            ("contract_update_grants", serialize_native(&update_grants, CONTRACT_UPDATE_V2_PREFIX)),
            ("mint_control", serialize(&mint)),
            ("coin_control", serialize(&coin)),
        ] {
            println!("SDK_GOLDEN {} {}", name, payload);
        }
        for number in [17u8, 18] {
            let bytes = [number, 0, 1]; // type, empty payload, Current sender
            assert!(postcard::from_bytes::<NetworkTXN>(&bytes).is_err());
            println!("SDK_GOLDEN retired_{}_network {}", number, base64::encode(&bytes));
        }
        // Coin and mint helpers retain their exact unprefixed wire bytes.
        assert_eq!(base64::decode(&serialize(&coin)).unwrap(),
                   vec![1, b'X', 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(base64::decode(&serialize(&mint)).unwrap(),
                   vec![1, b'X', 1, b'1', 1, b'Y']);
    }
}
