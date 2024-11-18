use crate::permission_key_ext::PermissionKeyExt;

pub type FileId = u32;

impl PermissionKeyExt for FileId {
    fn as_permission_key(&self) -> String {
        format!("file:{}", self)
    }
}
