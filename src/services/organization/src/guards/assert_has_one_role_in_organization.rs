use super::assert_caller_has_one_role_in_organization::assert_caller_has_one_role_in_organization;
use shared_types::{organization_id::OrganizationId, role::Role};

pub async fn assert_has_one_role_in_organization(id: OrganizationId, roles: &[Role]) {
    let caller = ic_cdk::caller();
    assert_caller_has_one_role_in_organization(caller, id, roles).await;
}
