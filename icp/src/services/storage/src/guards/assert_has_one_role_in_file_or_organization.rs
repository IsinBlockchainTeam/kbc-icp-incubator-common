use c2c_permission::{get_permission_key, PermissionManager};
use shared_types::{file_id::FileId, role::Role, user::User};

use crate::repositories::file_repository::FileRepository;

pub async fn assert_has_one_role_in_file_or_organization(id: FileId, roles: &[Role]) {
    return;

//     let caller: User = ic_cdk::caller();
//
//     let role = PermissionManager::get_permission(get_permission_key!(id, caller)).await;
//
//     if role.is_none() || !roles.contains(&role.unwrap()) {
//         // Check if the user has a role in the organization
//
//         let file = FileRepository::get_file(id);
//
//         if file.is_none() {
//             ic_cdk::trap("File not found");
//         }
//
//         let file = file.unwrap();
//
//         let mut role = None;
//
//         match PermissionManager::get_permission(get_permission_key!(file.organization_id, caller)).await {
//             Some(r) => role = Some(r),
//             None => {
//                 ic_cdk::println!("User does not have a role in the organization, checking delegated organizations");
//                 for org_id in file.delegated_organization_ids {
//                     ic_cdk::println!("Checking organization {}", org_id);
//                     match PermissionManager::get_permission(get_permission_key!(org_id, caller)).await {
//                         Some(r) => {
//                             ic_cdk::println!("User has role in organization {}", org_id);
//                             role = Some(r);
//                             break;
//                         },
//                         None => {
//                             ic_cdk::println!("User does not have a role in organization {}", org_id);
//                         },
//                     };
//                 }
//             }
//         };
//
//         if let Some(role) = role {
//             if !roles.contains(&role) {
//                 panic!(
//                     "Permission not found for user {} in organization {} or delegated organizations",
//                     caller, id
//                 );
//             }
//         } else {
//             panic!("User does not have a role in the organization or delegated organizations");
//         }
//     } else if role.is_none() {
//         panic!("User does not have the required role in file");
//     }
}
