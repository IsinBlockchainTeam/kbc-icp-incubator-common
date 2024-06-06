use crate::{
    guards::assert_has_one_role_in_file_or_organization::assert_has_one_role_in_file_or_organization,
    repositories::file_repository::FileRepository,
};
use c2c_permission::{get_permission_key, PermissionManager};
use guards::caller_is_authenticated;
use shared_types::{file_id::FileId, role::Role};

#[ic_cdk::update(guard = "caller_is_authenticated")]
async fn remove_file(file_id: FileId) -> Result<(), String> {
    assert_has_one_role_in_file_or_organization(file_id, &[Role::Editor, Role::Owner]).await;

    let keys = PermissionManager::get_subkeys(get_permission_key!(file_id))
        .await
        .unwrap();

    // Then delete all the permissions for the organization
    for key in keys {
        PermissionManager::delete_permission(format!("{}/{}", key.0, get_permission_key!(file_id)))
            .await?;
    }

    PermissionManager::delete_permission(get_permission_key!(file_id)).await?;

    FileRepository::remove_file(file_id);
    Ok(())
}
