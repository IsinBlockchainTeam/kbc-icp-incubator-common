use crate::{
    models::file_with_role::FileWithRole, repositories::file_repository::FileRepository,
};
use c2c_permission::{get_permission_key, PermissionManager};
use guards::caller_is_authenticated;
use shared_types::{file_id::FileId, organization_id::OrganizationId, user::User};
use shared_types::role::Role;
use crate::guards::assert_has_one_role_in_file_or_organization::assert_has_one_role_in_file_or_organization;
use std::panic;

async fn check_access(file_id: FileId, roles: &[Role]) -> bool {
    let result = panic::catch_unwind(|| {
        assert_has_one_role_in_file_or_organization(file_id, roles);
    });

    result.is_ok()
}

#[ic_cdk::query(composite = true, guard = "caller_is_authenticated")]
async fn get_files_for_organization(organization_id: OrganizationId) -> Vec<FileWithRole> {
    // assert_is_member_of_organization(organization_id).await;
    let caller: User = ic_cdk::caller();
    let mut result: Vec<FileWithRole> = Vec::new();

    ic_cdk::print(format!("Getting files for organization {}", organization_id));

    let files = FileRepository::get_files_for_organization(organization_id);

    for file in files {
        ic_cdk::print(format!("Getting role for file {}", file.id));

        if !check_access(file.id, &Role::values()).await {
            // If panic occurred, skip this file and continue with the next one
            continue;
        }

        // If no panic occurred, proceed with getting the role for the file
        let role = PermissionManager::get_permission(get_permission_key!(file.id as FileId, caller)).await;
        result.push(FileWithRole::new(file, role));
    }

    result
}
