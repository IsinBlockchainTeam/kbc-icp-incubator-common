use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::{borrow::Cow, fmt, hash::Hash};

use shared_types::{organization_id::OrganizationId, permission_key_ext::PermissionKeyExt};

#[derive(Clone, CandidType, Deserialize, Debug, Default)]
pub struct Organization {
    pub id: OrganizationId,
    pub name: String,
    pub description: String,
    pub verifiable_presentation: String,
}

impl PartialEq for Organization {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Organization {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Organization {}

impl Organization {
    pub fn new(id: OrganizationId, name: String, description: String, verifiable_presentation: String) -> Self {
        Self {
            id,
            name,
            description,
            verifiable_presentation,
        }
    }
}

impl fmt::Display for Organization {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format organization:id
        write!(f, "organization:{}", self.id)
    }
}

impl Storable for Organization {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl PermissionKeyExt for Organization {
    fn as_permission_key(&self) -> String {
        format!("organization:{}", self.id)
    }
}
