use candid::{CandidType, Principal};
use serde::Deserialize;
use shared_types::{file_id::FileId, organization_id::OrganizationId};

#[derive(Clone, CandidType, Deserialize)]
pub struct FileInfo {
    pub id: FileId,
    pub organization_id: OrganizationId,
    pub delegated_organization_ids: Vec<OrganizationId>,
    pub owner: Principal,
    pub name: String,
    pub mime_type: String,
    pub chunks: usize,
    pub total_size: u64,
    pub hash: [u8; 32],
}
