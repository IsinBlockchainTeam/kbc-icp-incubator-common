use std::collections::HashMap;

use crate::guards::assert_has_one_role_in_file_or_organization::assert_has_one_role_in_file_or_organization;
use c2c_permission::{get_permission_key, PermissionManager};
use guards::caller_is_authenticated;
use shared_types::{file_id::FileId, role::Role, user::User};

#[ic_cdk::query(composite = true, guard = "caller_is_authenticated")]
async fn get_file_members(id: FileId) -> HashMap<User, Role> {
    assert_has_one_role_in_file_or_organization(id, &[Role::Editor, Role::Owner]).await;

    let subkeys = PermissionManager::get_subkeys(get_permission_key!(id)).await;
    subkeys
        .unwrap()
        .into_iter()
        .filter(|(key, _)| key.starts_with("user:"))
        .map(|(key, role)| (key.replace("user:", ""), role))
        .map(|(key, role)| (key.parse().unwrap(), role))
        .collect()
}
