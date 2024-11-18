use c2c_permission::{get_permission_key, PermissionManager};
use shared_types::{organization_id::OrganizationId, role::Role, user::User};

pub async fn assert_has_one_role_in_organization(id: OrganizationId, roles: &[Role]) {
    return;

//     let caller: User = ic_cdk::caller();
//
//     let role = PermissionManager::get_permission(get_permission_key!(id, caller))
//         .await
//         .expect(
//             format!(
//                 "Permission not found for user {} in organization {}",
//                 caller, id
//             )
//             .as_str(),
//         );
//
//     if !roles.contains(&role) {
//         ic_cdk::trap("Caller does not have the required role in the organization");
//     }
}
