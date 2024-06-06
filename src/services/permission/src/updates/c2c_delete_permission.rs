use crate::repositories::permission_repository::PermissionRepository;

#[ic_cdk::update]
fn c2c_delete_permission(key: String) -> Result<(), String> {
    ic_cdk::print(format!("Deleting permission for key: {}", key));
    PermissionRepository::delete(&key)
}
