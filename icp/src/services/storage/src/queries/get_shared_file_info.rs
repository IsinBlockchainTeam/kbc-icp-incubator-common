use crate::{
    models::file_info::FileInfo,
    repositories::{file_repository::FileRepository, share_repository::SharedRepository},
};
use shared_types::file_id::FileId;

#[ic_cdk::query]
async fn get_shared_file_info(file_id: FileId, token: String) -> Result<FileInfo, String> {
    let share = SharedRepository::get_file_share(file_id, &token);

    if share.is_none() {
        return Err("Share not found".to_string());
    }

    let share = share.unwrap();

    if share.is_max_used() {
        return Err("Share is max used".to_string());
    }

    if share.is_expired() {
        return Err("Share is expired".to_string());
    }

    let file = FileRepository::get_file(file_id);

    Ok(file.unwrap().as_info())
}
