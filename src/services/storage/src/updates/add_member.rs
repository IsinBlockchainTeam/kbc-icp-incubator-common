use c2c_permission::{get_permission_key, PermissionManager};

use futures::join;
use guards::caller_is_authenticated;
use shared_types::{file_id::FileId, role::Role, user::User};

use crate::guards::assert_has_one_role_in_file_or_organization::assert_has_one_role_in_file_or_organization;

#[ic_cdk::update(guard = "caller_is_authenticated")]
async fn add_member(id: FileId, member: User, role: Role) -> Result<(), String> {
    assert_has_one_role_in_file_or_organization(id, &[Role::Owner]).await;

    if member == ic_cdk::caller() {
        return Err("You cannot add yourself as a member".to_string());
    }

    let result = join!(
        PermissionManager::set_permission(get_permission_key!(id, member), role),
        PermissionManager::set_permission(get_permission_key!(member, id), role)
    );

    if result.0.is_err() || result.1.is_err() {
        return Err("Failed to add member".to_string());
    }

    Ok(())
}
