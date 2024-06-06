use crate::{
    guards::assert_is_member_of_organization::assert_is_member_of_organization,
    models::organization::Organization,
    repositories::organization_repository::OrganizationRepository,
};
use guards::caller_is_authenticated;
use shared_types::organization_id::OrganizationId;

#[ic_cdk::query(composite = true, guard = "caller_is_authenticated")]
async fn get_organization(id: OrganizationId) -> Result<Organization, String> {
    assert_is_member_of_organization(id).await;

    let organization = OrganizationRepository::get_organization(id);

    match organization {
        Some(organization) => Ok(organization),
        None => Err("Organization not found".to_string()),
    }
}
