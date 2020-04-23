// <实现字典树>
// Implement a trie with insert, search, and startsWith methods.
// Example:
// Trie trie = new Trie();

// trie.insert("apple");
// trie.search("apple");   // returns true
// trie.search("app");     // returns false
// trie.startsWith("app"); // returns true
// trie.insert("app");
// trie.search("app");     // returns true

// Note:
// You may assume that all inputs are consist of lowercase letters a-z.
// All inputs are guaranteed to be non-empty strings.

// 解题思路：了解字典树特性
use crate::base::trie::Trie;
impl Trie {
    // 构建字典树(假设只存储26个英文字母)
    pub fn create(words: Vec<&str>) -> Trie {
        let mut root = Trie::new();
        for word in &words {
            root.insert(word.to_string());
        }
        return root;
    }

    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie {
            c: None,
            nodes: vec![None; 26],
            end: false,
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let chars: Vec<char> = word.chars().collect();
        let mut nodes = &mut self.nodes;
        for i in 0..chars.len() {
            let node: &mut Option<Trie> = &mut nodes[chars[i] as usize - 97];
            if node.is_none() {
                node.replace(Trie {
                    c: Some(chars[i]),
                    nodes: vec![None; 26],
                    end: i == chars.len() - 1,
                });
            } else {
                node.as_mut().unwrap().end = node.as_mut().unwrap().end || i == chars.len() - 1;
            }
            nodes = &mut node.as_mut().unwrap().nodes;
        }
    }

    /** Returns if the word is in the trie. */
    fn search(&mut self, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let mut nodes = &mut self.nodes;
        for i in 0..chars.len() {
            let node: &mut Option<Trie> = &mut nodes[chars[i] as usize - 97];
            if node.is_none() {
                return false;
            }
            if i == chars.len() - 1 && node.as_ref().unwrap().end {
                return true;
            }
            nodes = &mut node.as_mut().unwrap().nodes;
        }
        return false;
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&mut self, prefix: String) -> bool {
        let chars: Vec<char> = prefix.chars().collect();
        let mut nodes = &mut self.nodes;
        for i in 0..chars.len() {
            let node: &mut Option<Trie> = &mut nodes[chars[i] as usize - 97];
            if node.is_none() {
                return false;
            }
            nodes = &mut node.as_mut().unwrap().nodes;
        }
        return true;
    }
}

#[test]
fn run() {
    let words = vec!["how", "hi", "her", "hello", "so", "see"];
    let t: Trie = Trie::create(words);
    println!("{:?}", t);

    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
    assert_eq!(trie.search("app".to_string()), false);
    assert_eq!(trie.starts_with("app".to_string()), true);
    trie.insert("app".to_string());
    assert_eq!(trie.search("app".to_string()), true);
}
