/**
    CandidType is used for serialization. In order to deserialize,
    the CandidType and Serde's Deserialize trait.
**/
use candid::{candid_method, CandidType, Deserialize};
use ic_kit::{ic , Principal};
use ic_cdk_macros::*;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::string::String;