use std::collections::HashMap;

/// TrieNode is a node in a Trie.
pub struct TrieNode<'a, T> {
    children: HashMap<&'a str, TrieNode<'a, T>>,
    data: Option<T>,
}

impl<'a, T> Default for TrieNode<'a, T> {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            data: None,
        }
    }
}

/// Trie is a data structure that stores a set of strings.
/// It is used to find the longest match of a string.
///
/// # Example
///
/// ```rust
/// use rk_utils::Trie;
///
/// let mut trie = Trie::new();
/// trie.insert(vec!["a", "b", "c"], 1);
/// trie.insert(vec!["a", "b", "d"], 2);
///
/// assert_eq!(trie.find_longest_match(vec!["a", "b", "c", "d"]), Some(&1));
/// assert_eq!(trie.find_longest_match(vec!["a", "b", "d", "e"]), Some(&2));
/// ```
pub struct Trie<'a, T> {
    root: TrieNode<'a, T>,
}

impl<'a, T> Default for Trie<'a, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Trie<'a, T> {
    /// Create a new Trie.
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    /// Insert a path of nodes with data.
    pub fn insert(&mut self, path: Vec<&'a str>, data: T) {
        let mut node = &mut self.root;
        for key in path.iter() {
            node = node.children.entry(key).or_default();
        }
        node.data = Some(data);
    }

    /// Find the longest match of a path of nodes.
    /// It returns the data associated with the longest matched path.
    pub fn find_longest_match(&'a self, request_path: Vec<&'a str>) -> Option<&T> {
        let mut node = &self.root;
        let mut last_matched_data: Option<&T> = None;

        for key in request_path.iter() {
            if let Some(next_node) = node.children.get(key) {
                node = next_node;
                if node.data.is_some() {
                    last_matched_data = node.data.as_ref();
                }
            } else {
                break;
            }
        }

        last_matched_data
    }
}
