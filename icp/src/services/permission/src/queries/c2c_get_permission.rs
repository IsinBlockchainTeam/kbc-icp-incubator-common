use shared_types::role::Role;

use crate::repositories::permission_repository::PermissionRepository;

#[ic_cdk::query]
fn c2c_get_permission(key: String) -> Option<Role> {
    ic_cdk::print(format!("Getting permission for key: {}", key));
    PermissionRepository::get(&key)
}
