/**
    CandidType is used for serialization. In order to deserialize,
    the CandidType and Serde's Deserialize trait.

    A principal describes the security context of an identity, namely
    any identity that can be authenticated along with a specific role.
**/
use candid::{candid_method, CandidType, Deserialize};
use ic_cdk_macros::*;
use ic_kit::{ic, Principal};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::string::String;

/*
    It uses the MetaListPaths syntax to specify a list of traits to
    implement or paths to derive macros to process.
*/
#[derive(Deserialize, CandidType, Clone, Debug)]
struct Metadata {
    logo: String,
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u64,
    owner: Principal,
    fee: u64,
    fee_to: Principal,
}

#[derive(Deserialize, CandidType, Clone, Debug)]
struct TokenInfo {
    metadata: Metadata,
    fee_to: Principal,
    // status info
    history_size: usize,
    deploy_time: u64,
    holder_number: usize,
    cycles: u64,
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            logo: "".to_string(),
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0u8,
            total_supply: 0,
            owner: Principal::anonymous(),
            fee: 0,
            fee_to: Principal::anonymous(),
        }
    }
}

type Balances = HashMap<Principal, u64>;
type Allowances = HashMap<Principal, HashMap<Principal, u64>>;
type Ops = Vec<OpRecord>;

#[derive(Deserialize, CandidType)]
struct UpgradePayload {
    metadata: Metadata,
    balance: Vec<(Principal, u64)>,
    allow: Vec<(Principal, Vec<(Principal, u64)>)>,
}

#[derive(CandidType, Clone, Copy, Debug, PartialEq)]
enum Operation {
    Mint,
    Transfer,
    TransferFrom,
    Approve,
}

#[derive(CandidType, Clone, Debug)]
struct OpRecord {
    caller: Option<Principal>,
    op: Operation,
    index: usize,
    from: Principal,
    to: Principal,
    amount: u64,
    fee: u64,
    timestamp: u64,
}

#[derive(CandidType, Debug, PartialEq)]
enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
}
type TxReceipt = Result<usize, TxError>;

/*
    Collects data about the trannsaction.
    Makes the Ops Vector mutable
    Stores data received in the OpRecord
*/
fn add_record(
    caller: Option<Principal>,
    op: Operation,
    from: Principal,
    to: Principal,
    amount: u64,
    fee: u64,
    timestamp: u64,
) -> usize {
    let ops = ic::get_mut::<Ops>();
    let index = ops.len();
    ops.push(OpRecord {
        caller,
        op,
        index,
        from,
        to,
        amount,
        fee,
        timestamp,
    });
    index
}

/*
    Init marks a function to run before the main function
    This function stores the meta data of the token.
    It also passes the data into the add_record function that
    handles storing in the OpRecord struct.
    _ is used when the name doesn't matter.
*/
#[init]
#[candid_method(init)]
fn init(
    logo: String,
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u64,
    owner: Principal,
    fee: u64,
) {
    let metadata = ic::get_mut::<Metadata>();
    metadata.logo = logo;
    metadata.name = name;
    metadata.symbol = symbol;
    metadata.decimals = decimals;
    metadata.total_supply = total_supply;
    metadata.owner = owner;
    metadata.fee = fee;
    let balances = ic::get_mut::<Balances>();
    balances.insert(owner, total_supply);
    let _ = add_record(
        Some(owner),
        Operation::Mint,
        Principal::from_text("aaaaa-aa").unwrap(),
        owner,
        total_supply,
        0,
        ic::time(),
    );
}

fn _transfer(from: Principal, to: Principal, value: u64) {
    let balances = ic::get_mut::<Balances>();
    let from_balance = balance_of(from);
    let from_balance_new = from_balance - value;
    if from_balance_new != 0 {
        balances.insert(from, from_balance_new);
    } else {
        balances.remove(&from);
    }
    
    let to_balance = balance_of(to);
    let to_balance_new = to_balance + value;
    if to_balance_new != 0 {
        balances.insert(to, to_balance_new);
    }
}
