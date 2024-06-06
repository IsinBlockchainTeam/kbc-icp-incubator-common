use crate::{
    guards::assert_is_member_of_organization_or_file::assert_is_member_of_organization_or_file,
    models::file_chunk::FileChunk, repositories::file_repository::FileRepository,
};
use guards::caller_is_authenticated;
use shared_types::file_id::FileId;

#[ic_cdk::query(composite = true, guard = "caller_is_authenticated")]
async fn get_file_chunk(id: FileId, number: u32) -> Result<FileChunk, String> {
    assert_is_member_of_organization_or_file(id).await;

    let file = FileRepository::get_file(id).unwrap();

    if !file.chunks.contains_key(&number) {
        return Err("Chunk not found".to_string());
    }

    let chunk = file.chunks.get(&number).unwrap();

    Ok(chunk.to_owned())
}
