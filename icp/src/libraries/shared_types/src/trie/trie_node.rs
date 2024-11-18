use std::collections::HashMap;

use candid::CandidType;
use serde::Deserialize;

#[derive(Clone, CandidType, Deserialize, Debug)]
pub struct TrieNode<T> {
    pub children: HashMap<String, TrieNode<T>>,
    pub value: Option<T>,
}

impl<T> TrieNode<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn remove_child(&mut self, key: &str) -> Option<TrieNode<T>> {
        self.children.remove(key)
    }
}

impl<T> Default for TrieNode<T> {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            value: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_node_new() {
        let trie_node: TrieNode<String> = TrieNode::new();
        assert_eq!(trie_node.children.len(), 0);
        assert_eq!(trie_node.value, None);
    }

    #[test]
    fn test_trie_node_default() {
        let trie_node: TrieNode<String> = TrieNode::default();
        assert_eq!(trie_node.children.len(), 0);
        assert_eq!(trie_node.value, None);
    }

    #[test]
    fn test_trie_node_remove_child() {
        let mut trie_node: TrieNode<String> = TrieNode::new();
        trie_node.children.insert("a".to_string(), TrieNode::new());
        trie_node.children.insert("b".to_string(), TrieNode::new());
        trie_node.remove_child("a");
        assert_eq!(trie_node.children.len(), 1);
        assert_eq!(trie_node.children.contains_key("a"), false);
    }
}
