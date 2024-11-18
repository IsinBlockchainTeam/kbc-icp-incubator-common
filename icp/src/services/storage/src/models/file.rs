use std::{borrow::Cow, collections::HashMap};

use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;
use shared_types::{file_id::FileId, organization_id::OrganizationId};

use crate::FileChunk;

use super::file_info::FileInfo;

#[derive(Clone, CandidType, Deserialize)]
pub struct File {
    pub id: FileId,
    pub organization_id: OrganizationId,
    pub delegated_organization_ids: Vec<OrganizationId>,
    pub owner: Principal,
    pub name: String,
    pub mime_type: String,
    pub chunks: HashMap<u32, FileChunk>,
    pub total_size: u64,
    pub hash: [u8; 32],
}

impl Storable for File {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl File {
    pub fn as_info(&self) -> FileInfo {
        FileInfo {
            id: self.id,
            organization_id: self.organization_id,
            delegated_organization_ids: self.delegated_organization_ids.clone(),
            owner: self.owner,
            name: self.name.clone(),
            mime_type: self.mime_type.clone(),
            chunks: self.chunks.len(),
            total_size: self.total_size,
            hash: self.hash,
        }
    }
}
