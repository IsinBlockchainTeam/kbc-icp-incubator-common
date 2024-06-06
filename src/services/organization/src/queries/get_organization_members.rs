use std::collections::HashMap;

use crate::guards::assert_is_member_of_organization::assert_is_member_of_organization;
use c2c_permission::{get_permission_key, PermissionManager};
use guards::caller_is_authenticated;
use shared_types::{organization_id::OrganizationId, role::Role, user::User};

#[ic_cdk::query(composite = true, guard = "caller_is_authenticated")]
async fn get_organization_members(id: OrganizationId) -> HashMap<User, Role> {
    assert_is_member_of_organization(id).await;

    let subkeys = PermissionManager::get_subkeys(get_permission_key!(id)).await;
    subkeys
        .unwrap()
        .into_iter()
        .filter(|(key, _)| key.starts_with("user:"))
        .map(|(key, role)| (key.replace("user:", ""), role))
        .map(|(key, role)| (key.parse().unwrap(), role))
        .collect()
}
