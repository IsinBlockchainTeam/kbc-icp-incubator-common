use candid::CandidType;
use serde::Deserialize;
use serde_bytes::ByteBuf;

#[derive(Clone, CandidType, Deserialize)]
pub struct FileChunk {
    pub chunk: ByteBuf,
    pub hash: [u8; 32],
}
