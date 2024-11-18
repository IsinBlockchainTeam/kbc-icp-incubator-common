use std::{borrow::Cow, collections::HashMap};

use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{de::DeserializeOwned, Deserialize};

use super::trie_node::TrieNode;

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct Trie<T: CandidType> {
    root: TrieNode<T>,
}

impl<T: CandidType + Clone> Trie<T> {
    pub fn insert(&mut self, key: String, value: T) {
        let mut node = &mut self.root;

        // Split the string by / and insert each part into the trie
        for part in key.split('/') {
            node = node
                .children
                .entry(part.to_string())
                .or_insert(TrieNode::new());
        }

        node.value = Some(value);
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        let mut node = &self.root;

        // Match part only by / and return the value
        for part in key.split('/') {
            node = match node.children.get(part) {
                Some(n) => n,
                None => return None,
            };
        }

        node.value.as_ref()
    }

    // Function to get all the subkeys of a given key
    pub fn get_subkeys(&self, key: &str) -> Option<HashMap<String, T>> {
        let mut node = &self.root;

        // Match part only by / and return the value
        for part in key.split('/') {
            node = match node.children.get(part) {
                Some(n) => n,
                None => return None,
            };
        }

        let mut result: HashMap<String, T> = HashMap::new();

        for (key, value) in &node.children {
            if let Some(value) = &value.value {
                result.insert(key.clone(), value.clone());
            }
        }

        Some(result)
    }

    pub fn delete(&mut self, key: &str) -> Result<(), String> {
        // Remove the node with the key from the parent node
        let mut parts: Vec<String> = key.split('/').map(|s| s.to_string()).collect();

        let last_part = parts.pop().unwrap();

        let mut current = &mut self.root;

        for part in parts {
            if let Some(node) = current.children.get_mut(&part) {
                current = node;
            } else {
                return Err("Key not found".to_string());
            }
        }

        match current.remove_child(&last_part) {
            Some(_) => Ok(()),
            None => Err("Key not found".to_string()),
        }
    }
}

impl<T: CandidType + DeserializeOwned> Storable for Trie<T> {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl<T: CandidType> Default for Trie<T> {
    fn default() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }
}

// Implement tests for the Trie
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut trie = Trie::default();

        trie.insert("a".to_string(), "1".to_string());
        trie.insert("a/b".to_string(), "2".to_string());
        trie.insert("a/b/c".to_string(), "3".to_string());

        assert_eq!(trie.get("a").unwrap(), "1");
        assert_eq!(trie.get("a/b").unwrap(), "2");
        assert_eq!(trie.get("a/b/c").unwrap(), "3");
    }

    #[test]
    fn test_delete() {
        let mut trie = Trie::default();

        trie.insert("a".to_string(), "1".to_string());
        trie.insert("a/b".to_string(), "2".to_string());
        trie.insert("a/b/c".to_string(), "3".to_string());

        trie.delete("a/b/c").unwrap();

        assert_eq!(trie.get("a").unwrap(), "1");
        assert_eq!(trie.get("a/b").unwrap(), "2");
        assert_eq!(trie.get("a/b/c"), None);
    }

    #[test]
    fn test_get_subkeys() {
        let mut trie = Trie::default();

        trie.insert("a".to_string(), "1".to_string());
        trie.insert("a/b".to_string(), "2".to_string());
        trie.insert("a/b/c".to_string(), "3".to_string());

        // Test getting subkeys for "a"
        let subkeys_a = trie.get_subkeys("a");
        assert!(subkeys_a.is_some());
        let subkeys_a = subkeys_a.unwrap();
        assert_eq!(subkeys_a.len(), 1);
        assert_eq!(subkeys_a.get("b").unwrap(), &"2");

        // Test getting subkeys for "a/b"
        let subkeys_ab = trie.get_subkeys("a/b");
        assert!(subkeys_ab.is_some());
        let subkeys_ab = subkeys_ab.unwrap();
        assert_eq!(subkeys_ab.len(), 1);
        assert_eq!(subkeys_ab.get("c").unwrap(), &"3");

        // Test getting subkeys for "a/b/c"
        let subkeys_abc = trie.get_subkeys("a/b/c");
        assert!(subkeys_abc.is_some());
        let subkeys_abc = subkeys_abc.unwrap();
        assert_eq!(subkeys_abc.len(), 0); // No further subkeys
    }
}
