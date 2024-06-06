use crate::{
    memory::{get_memory, Memory, ORGANIZATIONS_MEMORY_ID},
    models::organization::Organization,
};
use ic_stable_structures::StableBTreeMap;
use shared_types::organization_id::OrganizationId;
use std::cell::RefCell;

thread_local! {
    static ORGANIZATIONS: RefCell<StableBTreeMap<OrganizationId, Organization, Memory>> = RefCell::new(StableBTreeMap::init(
        get_memory(ORGANIZATIONS_MEMORY_ID)
    ));
}

pub struct OrganizationRepository;

impl OrganizationRepository {
    pub fn create_organization(name: String, description: String, verifiable_presentation: String) -> Option<Organization> {
        ORGANIZATIONS.with(|organizations| {
            let mut organizations = organizations.borrow_mut();
            let id = organizations.len() as OrganizationId;

            let org = Organization::new(id, name, description, verifiable_presentation);
            organizations.insert(id, org.clone());
            Some(org)
        })
    }

    pub fn get_organization(id: OrganizationId) -> Option<Organization> {
        ORGANIZATIONS.with(|organizations| {
            let organizations = organizations.borrow();
            organizations.get(&id)
        })
    }

    pub fn get_verifiable_presentation(id: OrganizationId) -> Option<String> {
        ORGANIZATIONS.with(|organizations| {
            let organizations = organizations.borrow();
            organizations.get(&id).map(|org| org.verifiable_presentation.clone())
        })
    }

    pub fn update_organization(
        id: OrganizationId,
        name: String,
        description: String,
        verifiable_presentation: String,
    ) -> Option<Organization> {
        ORGANIZATIONS.with(|organizations| {
            let mut organizations = organizations.borrow_mut();
            match organizations.get(&id) {
                Some(mut organization) => {
                    organization.name = name;
                    organization.description = description;
                    organization.verifiable_presentation = verifiable_presentation;
                    organizations.insert(id, organization.clone());
                    Some(organization)
                }
                None => None,
            }
        })
    }

    pub fn delete_organization(id: OrganizationId) -> Option<Organization> {
        ORGANIZATIONS.with(|organizations| {
            let mut organizations = organizations.borrow_mut();

            organizations.remove(&id)
        })
    }
}
