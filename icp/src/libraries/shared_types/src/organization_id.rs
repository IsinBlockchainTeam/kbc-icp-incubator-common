use crate::permission_key_ext::PermissionKeyExt;

pub type OrganizationId = u128;

impl PermissionKeyExt for OrganizationId {
    fn as_permission_key(&self) -> String {
        format!("organization:{}", self)
    }
}
