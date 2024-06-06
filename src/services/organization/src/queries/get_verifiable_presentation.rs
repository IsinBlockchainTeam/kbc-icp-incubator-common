use crate::repositories::organization_repository::OrganizationRepository;
use guards::caller_is_authenticated;
use shared_types::organization_id::OrganizationId;

#[ic_cdk::query(composite = true, guard = "caller_is_authenticated")]
async fn get_verifiable_presentation(id: OrganizationId) -> Result<String, String> {
    let verifiable_presentation = OrganizationRepository::get_verifiable_presentation(id);

    match verifiable_presentation {
        Some(verifiable_presentation) => Ok(verifiable_presentation),
        None => Err("Verifiable presentation not found".to_string()),
    }
}
