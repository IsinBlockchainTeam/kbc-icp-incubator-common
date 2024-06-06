use crate::{
    guards::assert_is_member_of_organization_or_file::assert_is_member_of_organization_or_file,
    models::file_info::FileInfo, repositories::file_repository::FileRepository,
};
use guards::caller_is_authenticated;
use shared_types::file_id::FileId;

#[ic_cdk::query(composite = true, guard = "caller_is_authenticated")]
async fn get_file(file_id: FileId) -> FileInfo {
    assert_is_member_of_organization_or_file(file_id).await;

    let file = FileRepository::get_file(file_id);

    file.unwrap().as_info()
}
