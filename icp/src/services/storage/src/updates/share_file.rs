use crate::{
    guards::assert_has_one_role_in_file_or_organization::assert_has_one_role_in_file_or_organization,
    models::shared_link::SharedLink, repositories::share_repository::SharedRepository,
};
use guards::caller_is_authenticated;
use shared_types::{file_id::FileId, role::Role};

#[ic_cdk::update(guard = "caller_is_authenticated")]
async fn share_file(file_id: FileId, max_uses: Option<u64>, expires_at: Option<u64>) -> SharedLink {
    assert_has_one_role_in_file_or_organization(file_id, &[Role::Editor, Role::Owner]).await;
    let caller = ic_cdk::caller();

    SharedRepository::share_file(file_id, max_uses, expires_at, caller).await
}
