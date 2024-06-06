use super::assert_has_one_role_in_organization::assert_has_one_role_in_organization;
use shared_types::{organization_id::OrganizationId, role::Role};

pub async fn assert_is_member_of_organization(id: OrganizationId) {
    assert_has_one_role_in_organization(id, &Role::values()).await;
}
