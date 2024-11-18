use candid::Principal;

use crate::permission_key_ext::PermissionKeyExt;

pub type User = Principal;

impl PermissionKeyExt for User {
    fn as_permission_key(&self) -> String {
        format!("user:{}", self.to_text())
    }
}
