mod types;
use c2c_permission::PermissionManager;
use candid::Principal;
use ic_cdk::export_candid;
use models::organization::Organization;
use shared_types::{organization_id::OrganizationId, role::Role, user::User};
use std::collections::HashMap;

mod guards;
mod memory;
mod models;
mod queries;
mod repositories;
mod updates;

#[ic_cdk::init]
fn init(permission_canister_id: String) {
    ic_cdk::println!("Initializing organization canister");

    PermissionManager::init(Principal::from_text(permission_canister_id).unwrap());
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("Upgrading organization canister");
}

export_candid!();
