use std::collections::HashMap;

use crate::{
    models::organization::Organization,
    repositories::organization_repository::OrganizationRepository,
};
use c2c_permission::{get_permission_key, PermissionManager};
use guards::caller_is_authenticated;
use shared_types::{role::Role, user::User};

#[ic_cdk::query(composite = true, guard = "caller_is_authenticated")]
async fn get_user_organizations() -> Result<HashMap<Organization, Role>, String> {
    let caller: User = ic_cdk::caller();

    let subkeys: Option<HashMap<String, Role>> =
        PermissionManager::get_subkeys(get_permission_key!(caller)).await;

    let mut result: HashMap<Organization, Role> = HashMap::new();

    subkeys
        .unwrap()
        .into_iter()
        .filter(|(key, _)| key.starts_with("organization:"))
        .map(|(key, role)| (key.replace("organization:", ""), role))
        .for_each(|(key, role)| {
            let organization =
                OrganizationRepository::get_organization(key.parse().unwrap()).unwrap();

            result.insert(organization, role);
        });

    Ok(result)
}
