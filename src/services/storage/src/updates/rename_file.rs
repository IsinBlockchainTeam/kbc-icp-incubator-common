use crate::{
    guards::assert_has_one_role_in_file_or_organization::assert_has_one_role_in_file_or_organization,
    repositories::file_repository::FileRepository,
};
use guards::caller_is_authenticated;
use shared_types::{file_id::FileId, role::Role};

#[ic_cdk::update(guard = "caller_is_authenticated")]
async fn rename_file(file_id: FileId, new_name: String) -> Result<(), String> {
    assert_has_one_role_in_file_or_organization(file_id, &[Role::Editor, Role::Owner]).await;

    FileRepository::rename_file(file_id, new_name);

    Ok(())
}
