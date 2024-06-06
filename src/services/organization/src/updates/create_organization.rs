use crate::{
    models::organization::Organization,
    repositories::organization_repository::OrganizationRepository,
};

use c2c_permission::{get_permission_key, PermissionManager};
use futures::join;
use guards::caller_is_authenticated;
use shared_types::{role::Role, user::User};

#[ic_cdk::update(guard = "caller_is_authenticated")]
async fn create_organization(name: String, description: String, verifiable_presentation: String) -> Result<Organization, String> {
    let owner: User = ic_cdk::caller();

    let org = OrganizationRepository::create_organization(name, description, verifiable_presentation);

    match org {
        Some(org) => {
            let _ = join!(
                PermissionManager::set_permission(get_permission_key!(org, owner), Role::Owner),
                PermissionManager::set_permission(get_permission_key!(owner, org), Role::Owner)
            );

            Ok(org)
        }
        None => Err("Failed to create organization".to_string()),
    }
}
