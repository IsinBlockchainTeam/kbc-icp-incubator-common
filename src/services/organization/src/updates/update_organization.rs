use crate::{
    guards::assert_has_role_in_organization::assert_has_role_in_organization,
    models::organization::Organization,
    repositories::organization_repository::OrganizationRepository,
};

use guards::caller_is_authenticated;
use shared_types::{organization_id::OrganizationId, role::Role};

#[ic_cdk::update(guard = "caller_is_authenticated")]
async fn update_organization(
    id: OrganizationId,
    name: String,
    description: String,
    verifiable_presentation: String
) -> Result<Organization, String> {
    assert_has_role_in_organization(id, Role::Owner).await;

    OrganizationRepository::update_organization(id, name, description, verifiable_presentation)
        .ok_or("Organization not found".to_string())
}
