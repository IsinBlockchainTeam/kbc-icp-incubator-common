use c2c_permission::{get_permission_key, PermissionManager};
use futures::join;
use guards::caller_is_authenticated;
use shared_types::{file_id::FileId, organization_id::OrganizationId, role::Role};

use crate::{
    guards::assert_has_one_role_in_organization::assert_has_one_role_in_organization,
    repositories::file_repository::FileRepository,
};

#[ic_cdk::update(guard = "caller_is_authenticated")]
async fn create_file(
    organization_id: OrganizationId,
    delegated_organization_ids: Vec<OrganizationId>,
    name: String,
    mime_type: String,
    total_size: u64,
    hash: [u8; 32],
) -> FileId {
    // TODO: fix this check to allow also the other part to be able to upload files
    // assert_has_one_role_in_organization(organization_id, &[Role::Editor, Role::Owner]).await;

    let owner = ic_cdk::caller();
    let file_id =
        FileRepository::create_file(organization_id, delegated_organization_ids, owner, name, mime_type, total_size, hash);

    ic_cdk::print(format!("File created with id: {}", file_id));

    let _ = join!(
        PermissionManager::set_permission(get_permission_key!(file_id, owner), Role::Owner),
        PermissionManager::set_permission(get_permission_key!(owner, file_id), Role::Owner),
    );

    file_id
}
