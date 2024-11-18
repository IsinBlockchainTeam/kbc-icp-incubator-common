use std::collections::HashMap;

use shared_types::role::Role;

use crate::repositories::permission_repository::PermissionRepository;

#[ic_cdk::query]
fn c2c_get_subkeys(key: String) -> Option<HashMap<String, Role>> {
    ic_cdk::print(format!("Getting subkeys for key: {}", key));
    PermissionRepository::get_subkeys(&key)
}
