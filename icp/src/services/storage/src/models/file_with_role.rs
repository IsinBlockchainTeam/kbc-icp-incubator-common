use candid::CandidType;
use serde::Deserialize;
use shared_types::role::Role;

use super::{file::File, file_info::FileInfo};

#[derive(CandidType, Deserialize)]
pub struct FileWithRole {
    file: FileInfo,
    role: Option<Role>,
}

impl FileWithRole {
    pub fn new(file: File, role: Option<Role>) -> Self {
        Self {
            file: file.as_info(),
            role,
        }
    }
}
