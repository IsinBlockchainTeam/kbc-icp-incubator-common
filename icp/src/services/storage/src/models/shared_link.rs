use candid::{CandidType, Principal};
use serde::Deserialize;
use shared_types::file_id::FileId;

#[derive(Clone, CandidType, Deserialize)]
pub struct SharedLink {
    pub file_id: FileId,
    pub token: String,
    pub uses: u64,
    pub max_uses: Option<u64>,
    pub expires_at: Option<u64>,
    pub shared_by: Principal,
}

impl SharedLink {
    pub fn is_max_used(&self) -> bool {
        if let Some(max_uses) = self.max_uses {
            if max_uses != 0 && self.uses >= max_uses {
                return true;
            }
        }

        false
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            if expires_at != 0 && expires_at < ic_cdk::api::time() {
                return true;
            }
        }

        false
    }
}
