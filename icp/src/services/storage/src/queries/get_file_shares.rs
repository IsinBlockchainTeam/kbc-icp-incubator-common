use crate::{
    guards::assert_has_one_role_in_file_or_organization::assert_has_one_role_in_file_or_organization,
    models::shared_link::SharedLink, repositories::share_repository::SharedRepository,
};
use guards::caller_is_authenticated;
use shared_types::{file_id::FileId, role::Role};

#[ic_cdk::query(composite = true, guard = "caller_is_authenticated")]
async fn get_file_shares(file_id: FileId) -> Vec<SharedLink> {
    assert_has_one_role_in_file_or_organization(file_id, &[Role::Editor, Role::Owner]).await;

    SharedRepository::get_file_shares(file_id)
}
