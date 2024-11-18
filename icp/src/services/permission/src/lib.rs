use ic_cdk::export_candid;
use shared_types::role::Role;
use shared_types::trie::trie::Trie;
use std::collections::HashMap;

mod models;
mod queries;
mod repositories;
mod updates;

export_candid!();
