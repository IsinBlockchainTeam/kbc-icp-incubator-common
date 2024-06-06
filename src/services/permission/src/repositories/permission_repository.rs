use ic_stable_structures::{DefaultMemoryImpl, StableCell};
use shared_types::{role::Role, trie::trie::Trie};
use std::{cell::RefCell, collections::HashMap};

thread_local! {
    static PERMISSIONS: RefCell<StableCell<Trie<Role>, DefaultMemoryImpl>> = RefCell::new(StableCell::init(DefaultMemoryImpl::default(), Trie::default()).expect("Failed to initialize StableCell"));
}

pub struct PermissionRepository;

impl PermissionRepository {
    pub fn inspect_permissions() -> Trie<Role> {
        PERMISSIONS.with(|permissions| {
            let binding = permissions.borrow();
            let permissions = binding.get();

            permissions.clone()
        })
    }

    pub fn get(key: &str) -> Option<Role> {
        PERMISSIONS.with(|permissions| {
            let binding = permissions.borrow();
            let permissions = binding.get();

            permissions.get(&key).cloned()
        })
    }

    pub fn get_subkeys(key: &str) -> Option<HashMap<String, Role>> {
        PERMISSIONS.with(|permissions| {
            let binding = permissions.borrow();
            let permissions = binding.get();

            permissions.get_subkeys(&key)
        })
    }

    pub fn delete(key: &str) -> Result<(), String> {
        PERMISSIONS.with(|permissions| {
            let mut binding = permissions.borrow_mut();
            let mut permissions = binding.get().clone();

            let result = permissions.delete(&key);

            let _ = binding.set(permissions);

            result
        })
    }

    pub fn set(key: String, value: Role) -> Result<(), String> {
        PERMISSIONS.with(|permissions| {
            let mut binding = permissions.borrow_mut();
            let mut permissions = binding.get().clone();

            permissions.insert(key, value);

            let _ = binding.set(permissions);

            Ok(())
        })
    }
}
