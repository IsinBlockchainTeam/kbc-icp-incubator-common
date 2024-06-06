use crate::{
    models::file_chunk::FileChunk,
    repositories::{file_repository::FileRepository, share_repository::SharedRepository},
};
use shared_types::file_id::FileId;

#[ic_cdk::update]
async fn get_shared_file_chunk(
    file_id: FileId,
    number: u32,
    token: String,
) -> Result<FileChunk, String> {
    let share = SharedRepository::get_file_share(file_id, &token);

    if share.is_none() {
        return Err("Share not found".to_string());
    }

    let file = FileRepository::get_file(file_id).unwrap();

    if !file.chunks.contains_key(&number) {
        return Err("Chunk not found".to_string());
    }

    let chunk = file.chunks.get(&number).unwrap();

    if number == file.chunks.len() as u32 - 1 {
        if SharedRepository::use_file_share(file_id, &token).is_none() {
            return Err("Share not found".to_string());
        }
    }

    Ok(chunk.to_owned())
}
