use crate::{
    guards::assert_has_one_role_in_file_or_organization::assert_has_one_role_in_file_or_organization,
    repositories::share_repository::SharedRepository,
};
use guards::caller_is_authenticated;
use shared_types::{file_id::FileId, role::Role};

#[ic_cdk::update(guard = "caller_is_authenticated")]
async fn remove_share(file_id: FileId, token: String) {
    assert_has_one_role_in_file_or_organization(file_id, &[Role::Editor, Role::Owner]).await;

    SharedRepository::remove_share(file_id, token)
}
