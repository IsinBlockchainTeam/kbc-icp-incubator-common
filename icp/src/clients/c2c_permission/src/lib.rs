use candid::Principal;
use shared_types::role::Role;
use std::{cell::RefCell, collections::HashMap};

#[macro_export]
macro_rules! get_permission_key {
    ($first:expr $(, $rest:expr)*) => {{
        use shared_types::permission_key_ext::PermissionKeyExt;

        fn assert_permission_key<T: PermissionKeyExt>(_: &T) {}
        assert_permission_key(&$first);
        $(
            assert_permission_key(&$rest);
        )*

        let mut key = String::from($first.as_permission_key());
        $(
            key.push('/');
            key.push_str(&$rest.as_permission_key());
        )*
        key
    }};
}

thread_local! {
    static PERMISSION_CANISTER_ID: RefCell<Option<Principal>> = RefCell::new(None);
}

pub struct PermissionManager;

impl PermissionManager {
    pub fn init(canister_id: Principal) {
        ic_cdk::print(format!(
            "Initializing permission canister with id: {}",
            canister_id
        ));
        PERMISSION_CANISTER_ID.with(|id| {
            *id.borrow_mut() = Some(canister_id);
        });
    }

    fn get_canister_id() -> Option<Principal> {
        PERMISSION_CANISTER_ID.with(|id| id.borrow().clone())
    }

    pub async fn get_permission(key: String) -> Option<Role> {
        let canister_id = Self::get_canister_id().unwrap();

        let (role,) =
            ic_cdk::call::<(String,), (Option<Role>,)>(canister_id, "c2c_get_permission", (key,))
                .await
                .unwrap();

        role
    }

    pub async fn get_subkeys(key: String) -> Option<HashMap<String, Role>> {
        let canister_id = Self::get_canister_id().unwrap();

        let (subkeys,) = ic_cdk::call::<(String,), (Option<HashMap<String, Role>>,)>(
            canister_id,
            "c2c_get_subkeys",
            (key,),
        )
        .await
        .unwrap();

        subkeys
    }

    pub async fn set_permission(key: String, role: Role) -> Result<(), String> {
        let canister_id = Self::get_canister_id().unwrap();

        let (result,) = ic_cdk::call::<(String, Role), (Result<(), String>,)>(
            canister_id,
            "c2c_set_permission",
            (key, role),
        )
        .await
        .unwrap();

        result
    }

    pub async fn delete_permission(key: String) -> Result<(), String> {
        let canister_id = Self::get_canister_id().unwrap();

        let (result,) = ic_cdk::call::<(String,), (Result<(), String>,)>(
            canister_id,
            "c2c_delete_permission",
            (key,),
        )
        .await
        .unwrap();

        result
    }
}
