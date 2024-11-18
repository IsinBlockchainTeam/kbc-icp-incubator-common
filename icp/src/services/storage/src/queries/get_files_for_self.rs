use std::collections::HashMap;

use crate::{models::file_with_role::FileWithRole, repositories::file_repository::FileRepository};
use c2c_permission::{get_permission_key, PermissionManager};
use guards::caller_is_authenticated;
use shared_types::{role::Role, user::User};

#[ic_cdk::query(composite = true, guard = "caller_is_authenticated")]
async fn get_files_for_self() -> Vec<FileWithRole> {
    let caller: User = ic_cdk::caller();
    let mut result: Vec<FileWithRole> = Vec::new();

    let subkeys: Option<HashMap<String, Role>> =
        PermissionManager::get_subkeys(get_permission_key!(caller)).await;

    // Keep only the keys that start with "file:"
    subkeys
        .unwrap()
        .into_iter()
        .filter(|(key, _)| key.starts_with("file:"))
        .map(|(key, role)| (key.replace("file:", ""), role))
        .for_each(|(key, role)| {
            let file = FileRepository::get_file(key.parse().unwrap()).unwrap();

            result.push(FileWithRole::new(file, Some(role)));
        });

    result
}
