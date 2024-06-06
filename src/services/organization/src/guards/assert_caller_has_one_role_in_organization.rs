use c2c_permission::{get_permission_key, PermissionManager};

use crate::repositories::organization_repository::OrganizationRepository;
use shared_types::{organization_id::OrganizationId, role::Role, user::User};

pub async fn assert_caller_has_one_role_in_organization(
    caller: User,
    id: OrganizationId,
    roles: &[Role],
) -> Role {
    // Get organization
    let organization = OrganizationRepository::get_organization(id);

    if organization.is_none() {
        ic_cdk::trap("Organization does not exist");
    }

    let organization = organization.unwrap();

    let role = PermissionManager::get_permission(get_permission_key!(organization, caller)).await;

    if role.is_none() {
        ic_cdk::trap("Caller is not a member of the organization");
    }

    let role = role.unwrap();

    // Check if the caller has the required role
    if !roles.contains(&role) {
        ic_cdk::trap("Caller does not have the required role in the organization");
    }

    role
}
