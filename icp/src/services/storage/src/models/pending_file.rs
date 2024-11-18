use std::{borrow::Cow, collections::HashMap};

use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{storable::Bound, Storable};
use serde::Deserialize;
use shared_types::{file_id::FileId, organization_id::OrganizationId};

use crate::FileChunk;

#[derive(Clone, CandidType, Deserialize)]
pub struct PendingFile {
    pub id: FileId,
    pub organization_id: OrganizationId,
    pub delegated_organization_ids: Vec<OrganizationId>,
    pub owner: Principal,
    pub name: String,
    pub mime_type: String,
    pub current_size: u64,
    pub total_size: u64,
    pub hash: [u8; 32],
    pub chunks: HashMap<u32, FileChunk>,
}

impl Storable for PendingFile {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
