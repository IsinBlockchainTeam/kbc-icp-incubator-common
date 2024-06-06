use c2c_permission::PermissionManager;
use candid::Principal;
use ic_cdk::export_candid;
use models::{
    file_chunk::FileChunk, file_info::FileInfo, file_with_role::FileWithRole,
    shared_link::SharedLink,
};
use shared_types::file_id::FileId;
use shared_types::organization_id::OrganizationId;
use shared_types::role::Role;
use shared_types::user::User;
use std::collections::HashMap;

mod guards;
mod memory;
mod models;
mod queries;
mod repositories;
mod types;
mod updates;

#[ic_cdk::init]
fn init(permission_canister_id: String) {
    ic_cdk::println!("Initializing storage canister");

    PermissionManager::init(Principal::from_text(permission_canister_id).unwrap());
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("Upgrading storage canister");
}

export_candid!();
