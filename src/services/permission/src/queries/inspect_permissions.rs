use shared_types::{role::Role, trie::trie::Trie};

use crate::repositories::permission_repository::PermissionRepository;

// TODO(@filippofinke): Remove in the future
#[ic_cdk::query]
fn inspect_permissions() -> Trie<Role> {
    PermissionRepository::inspect_permissions()
}
