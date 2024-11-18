use shared_types::role::Role;

use crate::repositories::permission_repository::PermissionRepository;

#[ic_cdk::update]
fn c2c_set_permission(key: String, value: Role) -> Result<(), String> {
    ic_cdk::print(format!(
        "Setting permission for key: {} to value: {:?}",
        key, value
    ));
    PermissionRepository::set(key, value)
}
