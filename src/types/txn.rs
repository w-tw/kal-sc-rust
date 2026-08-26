use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TxnSender {
    Delegate {
        sc_name: String,
        sc_instance: String,
    },
    Current,
    ContractBefore,
    OriginalContract,
    User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct NetworkTXN {
    pub(crate) txn_type: TransactionType,
    pub(crate) payload: String,
    pub(crate) sender: TxnSender,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransactionType {
    #[default]
    UnknownType,
    CoinType,
    MintType,
    ItemMintType,
    ContractTxnType,
    VoteType,
    ProposalType,
    SmartContractType,
    SmartContractExecuteType,
    ExpenseRatioType,
    NftType,
    UpdateContractType,
    ValidatorRegistrationType,
    ValidatorHeartbeatType,
    ProposalResultType,
    DelegatedVotingType,
    RevokeType,
    QuashType,
    FastQuorumType,
    ComplianceType,
    SbtBurnType,
    RequiredVersion,
    SmartContractInstantiateType,
    AllowanceType,
    ProposalCancelType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ContractUpdateTXN {
    pub contract_id: String,
    pub contract_version: u64,
    pub name: Option<String>,
    pub governance: Option<Governance>,
    pub restricted_keys: Vec<RestrictedKey>,
    pub contract_fees: Option<ContractFees>,
    pub custom_parameters: Vec<KeyValuePair>,
    pub expense_ratio: Vec<ExpenseRatio>,
    pub token_compliance: Vec<TokenCompliance>,
    pub kyc_status: Option<bool>,
    pub immutable_kyc_status: Option<bool>,
    pub quash_threshold: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InstrumentContractTXN {
    pub contract_version: u64,
    pub symbol: String,
    pub name: String,
    pub governance: Option<Governance>,
    pub restricted_keys: Vec<RestrictedKey>,
    pub max_supply: Option<String>,
    pub contract_fees: Option<ContractFees>,
    pub premint_wallets: Vec<PreMintWallet>,
    pub coin_denomination: Option<CoinDenomination>,
    pub custom_parameters: Vec<KeyValuePair>,
    pub contract_id: String,
    pub expense_ratio: Vec<ExpenseRatio>,
    pub contract_type: ContractType,
    pub update_contract_fees: bool,
    pub update_expense_ratio: bool,
    pub quash_threshold: Option<u32>,
    pub token_compliance: Vec<TokenCompliance>,
    pub kyc_status: bool,
    pub immutable_kyc_status: bool,
    pub max_supply_release: Vec<MaxSupplyRelease>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PreMintWallet {
    pub address: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CoinDenomination {
    pub denomination_name: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MaxSupplyRelease {
    pub release_date: ProtoTimestamp,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContractType {
    #[default]
    Token,
    Nft,
    Sbt,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Governance {
    pub governance_type: GovernanceType,
    pub regular_quorum: u32,
    pub fast_quorum: Option<u32>,
    pub voting_instrument: Vec<String>,
    pub threshold: u32,
    pub chicken_dinner: Option<bool>,
    pub allow_multi: bool,
    pub voting_period: Option<u32>,
    pub allowed_proposal_instrument: Vec<String>,
    pub proposal_period: Option<ProposalPeriod>,
    pub stage_length: Vec<Stage>,
    pub start_timestamp: Option<ProtoTimestamp>,
    pub max_approved: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Stage {
    pub length: u32,
    pub period: ProposalPeriod,
    pub break_stage: bool,
    pub max_approved: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RestrictedKey {
    pub public_key: PublicKey,
    pub time_delay: i64,
    pub global: bool,
    pub update_contract: bool,
    pub transfer: bool,
    pub quash: bool,
    pub mint: bool,
    pub vote: bool,
    pub propose: bool,
    pub compliance: bool,
    pub expense_ratio: bool,
    pub revoke: bool,
    pub key_weight: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PublicKey {
    /// Base58-encoded single public key (e.g. `"A_c_7kdro2TE..."`).
    pub single: String,
    pub multi: Option<MultiKey>,
    /// Smart-contract auth identifier in the form `"sc_<contract>_<instance>"`,
    /// e.g. `"sc_sc_test_proxy_1"`. Used to identify a smart contract as a
    /// restricted-key holder.
    pub smart_contract_auth: Option<String>,
    /// Governance auth identifier.
    pub governance_auth: Option<String>,
}

impl PublicKey {
    /// Build a `PublicKey` that authorizes a smart contract, identified by
    /// its `name` and optional `instance` (defaults to `"1"`).
    ///
    /// Equivalent to populating `smart_contract_auth` with the string
    /// returned by [`crate::wallet::sc_auth`] and leaving all other fields
    /// at their defaults.
    pub fn sc(contract: &str, instance: Option<&str>) -> Self {
        let inst = instance.unwrap_or("1");
        PublicKey {
            smart_contract_auth: Some(format!("sc_{contract}_{inst}")),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MultiKey {
    /// Base58-encoded public keys, with class prefix (e.g. `"c1_<key>"`).
    pub public_keys: Vec<String>,
    pub signatures: Vec<Vec<u8>>,
    pub multi_patterns: Vec<MultiPatterns>,
    pub hash_tokens: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MultiPatterns {
    pub class: Vec<u32>,
    pub required: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ContractFees {
    pub fee: String,
    /// Base58-encoded fee recipient wallet address.
    pub fee_address: Option<String>,
    pub burn: String,
    pub validator: String,
    pub allowed_fee_instrument: Vec<String>,
    pub contract_fee_type: ContractFeeType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct KeyValuePair {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ExpenseRatio {
    pub day: u32,
    pub month: u32,
    pub percent: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TokenCompliance {
    pub compliance: Vec<Compliance>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Compliance {
    pub contract_id: String,
    pub compliance_level: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProtoTimestamp {
    pub seconds: i64,
    pub nanos: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContractFeeType {
    #[default]
    None,
    Fixed,
    CurEquivalent,
    Percentage,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GovernanceType {
    #[default]
    Adaptive,
    Staged,
    Cycle,
    Staggered,
    Remove,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProposalPeriod {
    #[default]
    Days,
    Months,
}

/// Full coin-transfer transaction, mirroring the network's `CoinTXN` proto
/// (minus `base`, which the network fills in from the transaction context).
///
/// Use this when you need the full expressiveness of a coin transfer: multiple
/// inputs, multiple outputs, explicit fee routing, etc. For the common
/// "send X of Y to one recipient" case, prefer [`crate::transfer::send`] from
/// the `transfer` module, which handles all of this internally.
///
/// # Address / key fields
///
/// Every wallet address or public key in this transaction — including
/// `output_transfers[i].wallet_address` and `auth.public_key[i].single` — is
/// passed as a base58 `String`. The network decodes it on its side. Raw
/// binary fields (signatures, hashes) stay as `Vec<u8>`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CoinTXN {
    pub contract_id: String,
    /// Sender authorization. For smart-contract-originated transfers the
    /// network populates `public_key`, `signature`, and `nonce` from the
    /// `TxnSender` context — callers typically leave this at `Default`.
    pub auth: TransferAuthentication,
    pub input_transfers: Vec<InputTransfers>,
    pub output_transfers: Vec<OutputTransfers>,
    /// Contract used to pay this transfer's contract-level fee, if the
    /// token being transferred has one configured. Leave as `None` when
    /// transferring a token without contract fees.
    pub contract_fee_id: Option<String>,
    pub contract_fee_amount: Option<String>,
}

/// Sender authentication for a `CoinTXN`. Mirrors the network's
/// `TransferAuthentication` proto. Each position in `public_key`,
/// `signature`, and `nonce` refers to the same signer.
///
/// For SC-originated transfers the network populates every field here from
/// the `TxnSender` context; callers typically leave this at `Default`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TransferAuthentication {
    pub public_key: Vec<PublicKey>,
    pub signature: Vec<Vec<u8>>,
    pub nonce: Vec<u64>,
    /// Base58-encoded allowance wallet addresses.
    pub allowance_address: Vec<String>,
    pub allowance_nonce: Vec<u64>,
}

/// Describes which signer (by index into `TransferAuthentication.public_key`)
/// is sending, how much, and how fees are split. Mirrors `InputTransfers`.
///
/// `fee_percent` and `contract_fee_percent` use a fixed-point scale where
/// `100_000_000` = 100%.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InputTransfers {
    pub index: u64,
    pub amount: String,
    pub fee_percent: u32,
    pub contract_fee_percent: Option<u32>,
}

/// A single coin-transfer output. Mirrors `OutputTransfers`.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OutputTransfers {
    /// Base58-encoded recipient wallet address.
    pub wallet_address: String,
    pub amount: String,
    pub memo: Option<String>,
}

/// Full mint transaction, mirroring the network's `MintTXN` proto (minus
/// `base`, which the network fills in).
///
/// For the common "mint X to Y" case, prefer [`crate::transfer::mint`] from
/// the `transfer` module.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MintTXN {
    pub contract_id: String,
    pub amount: String,
    /// Base58-encoded recipient wallet address.
    pub recipient_address: String,
}
