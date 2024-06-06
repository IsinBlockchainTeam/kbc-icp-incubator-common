use crate::{
    memory::{get_memory, Memory, SHARED_FILES_MEMORY_ID},
    models::{shared_file::SharedFile, shared_link::SharedLink},
};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use candid::Principal;
use ic_stable_structures::BTreeMap;
use shared_types::file_id::FileId;
use std::cell::RefCell;

pub struct SharedRepository;

thread_local! {
    static SHARED_FILES: RefCell<BTreeMap<FileId, SharedFile, Memory>> = RefCell::new(BTreeMap::init(get_memory(SHARED_FILES_MEMORY_ID)));
}

impl SharedRepository {
    pub fn get_file_shares(file_id: FileId) -> Vec<SharedLink> {
        SHARED_FILES.with(|s| {
            let shared_files = s.borrow();
            shared_files
                .get(&file_id)
                .map(|f| f.links)
                .unwrap_or_default()
        })
    }

    pub fn get_file_share(file_id: FileId, token: &String) -> Option<SharedLink> {
        SHARED_FILES.with(|s| {
            let shared_files = s.borrow();
            shared_files.get(&file_id).and_then(|f| {
                f.links
                    .iter()
                    .find(|link| link.file_id == file_id && link.token == *token)
                    .cloned()
            })
        })
    }

    // TODO(@filippofinke): See for borrowing and data
    pub fn use_file_share(file_id: FileId, token: &String) -> Option<SharedLink> {
        SHARED_FILES.with(|s| {
            let mut shared_files = s.borrow_mut();
            if let Some(mut shared_file) = shared_files.get(&file_id) {
                let mut links = shared_file.links.clone();
                let link = shared_file
                    .links
                    .iter_mut()
                    .find(|link| link.file_id == file_id && link.token == *token);

                links.retain(|l| l.token != *token);

                if let Some(link) = link {
                    if let Some(max_uses) = link.max_uses {
                        if max_uses != 0 && link.uses >= max_uses {
                            return None;
                        }
                    }

                    if let Some(expires_at) = link.expires_at {
                        if expires_at != 0 && expires_at < ic_cdk::api::time() {
                            return None;
                        }
                    }

                    link.uses += 1;
                    links.push(link.clone());

                    shared_files.insert(file_id, SharedFile { links });

                    return Some(link.clone());
                }
            }

            None
        })
    }

    pub fn remove_share(file_id: FileId, token: String) {
        SHARED_FILES.with(|s| {
            let mut shared_files = s.borrow_mut();
            if let Some(mut shared_file) = shared_files.get(&file_id) {
                shared_file.links.retain(|l| l.token != token);

                shared_files.insert(file_id, shared_file);
            }
        })
    }

    pub async fn share_file(
        file_id: FileId,
        max_uses: Option<u64>,
        expires_at: Option<u64>,
        shared_by: Principal,
    ) -> SharedLink {
        let (bytes,): (Vec<u8>,) = ic_cdk::call(Principal::management_canister(), "raw_rand", ())
            .await
            .expect("Failed to call raw_rand");

        SHARED_FILES.with(|s| {
            let mut shared_files = s.borrow_mut();
            let shared_file = shared_files.get(&file_id);
            let mut shared_file = shared_file.unwrap_or_else(|| SharedFile { links: Vec::new() });

            let token: String = URL_SAFE.encode(&bytes);

            let shared_link = SharedLink {
                file_id,
                token,
                max_uses,
                uses: 0,
                expires_at,
                shared_by,
            };

            shared_file.links.push(shared_link.clone());

            shared_files.insert(file_id, shared_file);

            shared_link
        })
    }
}
