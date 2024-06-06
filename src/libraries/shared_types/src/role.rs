use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq, Copy)]
pub enum Role {
    Editor,
    Viewer,
    Owner,
}

impl Role {
    pub fn values() -> [Role; 3] {
        [Role::Editor, Role::Viewer, Role::Owner]
    }
}
