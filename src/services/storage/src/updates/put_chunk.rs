use crate::{models::file_chunk::FileChunk, repositories::file_repository::FileRepository};
use guards::caller_is_authenticated;
use shared_types::file_id::FileId;

#[ic_cdk::update(guard = "caller_is_authenticated")]
async fn put_chunk(file_id: FileId, chunk_number: u32, chunk: FileChunk) -> Result<(), String> {
    ic_cdk::print(format!("Putting chunk {} for file {}", chunk_number, file_id));
    let caller = ic_cdk::caller();
    let file = FileRepository::get_pending_file(file_id).ok_or("File not found")?;

    // Check if caller is owner of the file
    if file.owner != caller {
        return Err("Caller is not the owner of the file".to_string());
    }

    FileRepository::put_chunk(file_id, chunk_number, chunk)
}
