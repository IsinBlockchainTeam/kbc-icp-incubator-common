use candid::Principal;
use constant_time_eq::constant_time_eq;
use ic_stable_structures::BTreeMap;
use serde_bytes::ByteBuf;
use sha2::{Digest, Sha256};
use shared_types::{file_id::FileId, organization_id::OrganizationId};
use std::{cell::RefCell, collections::HashMap};

use crate::{
    memory::{get_memory, Memory, FILES_MEMORY_ID, PENDING_FILES_MEMORY_ID},
    models::{file::File, file_chunk::FileChunk, pending_file::PendingFile},
};

pub struct FileRepository;

thread_local! {
    static FILES: RefCell<BTreeMap<FileId, File, Memory>> = RefCell::new(BTreeMap::init(get_memory(FILES_MEMORY_ID)));
    static PENDING_FILES: RefCell<BTreeMap<FileId, PendingFile, Memory>> = RefCell::new(BTreeMap::init(get_memory(PENDING_FILES_MEMORY_ID)));
}

impl FileRepository {
    pub fn create_file(
        organization_id: OrganizationId,
        delegated_organization_ids: Vec<OrganizationId>,
        owner: Principal,
        name: String,
        mime_type: String,
        total_size: u64,
        hash: [u8; 32],
    ) -> FileId {
        PENDING_FILES.with(|pending_files| {
            let mut pending_files = pending_files.borrow_mut();

            let file_id = FILES.with(|files| {
                let files = files.borrow();
                let file_id = files.len() as FileId;
                file_id
            });

            pending_files.insert(
                file_id,
                PendingFile {
                    id: file_id,
                    organization_id,
                    delegated_organization_ids,
                    name,
                    mime_type,
                    owner,
                    hash,
                    chunks: HashMap::new(),
                    current_size: 0,
                    total_size,
                },
            );
            file_id
        })
    }

    pub fn put_chunk(file_id: FileId, chunk_number: u32, chunk: FileChunk) -> Result<(), String> {
        PENDING_FILES.with(|pending_files| {
            let mut pending_files = pending_files.borrow_mut();
            let mut pending_file = pending_files.get(&file_id).unwrap();

            if pending_file.chunks.contains_key(&chunk_number) {
                return Err("Chunk already exists".to_string());
            }

            // Validate chunk hash
            let mut hasher = Sha256::new();
            hasher.update(&chunk.chunk);
            let hash = hasher.finalize();

            if !constant_time_eq(&hash, &chunk.hash) {
                return Err("Invalid chunk hash".to_string());
            }

            pending_file.current_size += chunk.chunk.len() as u64;
            pending_file.chunks.insert(chunk_number, chunk);

            if pending_file.current_size == pending_file.total_size {
                let mut bytes = ByteBuf::new();

                let chunks = pending_file.chunks.len() as u32;

                for i in 0..chunks {
                    let chunk = pending_file.chunks.get(&i).unwrap();
                    bytes.extend_from_slice(&chunk.chunk);
                }

                // Validate file hash
                let mut hasher = Sha256::new();
                hasher.update(&bytes);
                let hash = hasher.finalize();

                if !constant_time_eq(&hash, &pending_file.hash) {
                    return Err("Invalid file hash".to_string());
                }

                let file = File {
                    id: file_id,
                    organization_id: pending_file.organization_id,
                    delegated_organization_ids: pending_file.delegated_organization_ids.clone(),
                    owner: pending_file.owner,
                    name: pending_file.name.clone(),
                    mime_type: pending_file.mime_type.clone(),
                    chunks: pending_file.chunks,
                    total_size: pending_file.total_size,
                    hash: pending_file.hash,
                };

                FILES.with(|files| {
                    let mut files = files.borrow_mut();
                    files.insert(file_id, file);
                });

                pending_files.remove(&file_id);
            } else {
                pending_files.insert(file_id, pending_file);
            }
            ic_cdk::print(format!("Completed chunk upload. chunk {} for file {}", chunk_number, file_id));
            Ok(())
        })
    }

    pub fn get_pending_file(file_id: FileId) -> Option<PendingFile> {
        PENDING_FILES.with(|pending_files| {
            let pending_files = pending_files.borrow();
            pending_files.get(&file_id)
        })
    }

    pub fn get_file(file_id: FileId) -> Option<File> {
        FILES.with(|files| {
            let files = files.borrow();
            files.get(&file_id)
        })
    }

    pub fn get_files_for_organization(organization_id: OrganizationId) -> Vec<File> {
        FILES.with(|files| {
            let files = files.borrow();

            let mut organization_files = Vec::new();

            for (_file_id, file) in files.iter() {
                if file.organization_id == organization_id {
                    organization_files.push(file);
                }
            }

            organization_files
        })
    }

    pub fn rename_file(file_id: FileId, name: String) -> Option<File> {
        FILES.with(|files| {
            let mut files = files.borrow_mut();
            let file = files.get(&file_id);

            if let Some(mut file) = file {
                file.name = name;
                files.insert(file_id, file.clone());
                Some(file)
            } else {
                None
            }
        })
    }

    pub fn remove_file(file_id: FileId) -> Option<File> {
        FILES.with(|files| {
            let mut files = files.borrow_mut();
            files.remove(&file_id)
        })
    }
}
