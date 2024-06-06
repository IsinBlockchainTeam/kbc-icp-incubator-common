use super::assert_has_one_role_in_organization::assert_has_one_role_in_organization;
use shared_types::{organization_id::OrganizationId, role::Role};

pub async fn assert_has_role_in_organization(id: OrganizationId, role: Role) {
    assert_has_one_role_in_organization(id, &[role]).await;
}
