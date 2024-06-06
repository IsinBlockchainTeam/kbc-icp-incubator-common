use crate::{
    guards::assert_has_role_in_organization::assert_has_role_in_organization,
    repositories::organization_repository::OrganizationRepository,
};
use c2c_permission::{get_permission_key, PermissionManager};
use guards::caller_is_authenticated;
use shared_types::{
    organization_id::OrganizationId, permission_key_ext::PermissionKeyExt, role::Role,
};

#[ic_cdk::update(guard = "caller_is_authenticated")]
async fn delete_organization(id: OrganizationId) -> Result<(), String> {
    assert_has_role_in_organization(id, Role::Owner).await;

    let result = OrganizationRepository::delete_organization(id);

    match result {
        Some(org) => {
            // First get all the members of the organization
            let keys = PermissionManager::get_subkeys(get_permission_key!(org))
                .await
                .unwrap();

            // Then delete all the permissions for the organization
            for key in keys {
                PermissionManager::delete_permission(format!(
                    "{}/{}",
                    key.0,
                    org.as_permission_key()
                ))
                .await?;
            }

            PermissionManager::delete_permission(get_permission_key!(org)).await?;

            Ok(())
        }
        None => Err("Failed to delete organization".to_string()),
    }
}
