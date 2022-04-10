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
            nodes: vec![None; 26],
            end: false,
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut trie = self;
        let cs: Vec<char> = word.chars().collect();
        for i in 0..cs.len() {
            let index = cs[i] as usize - 'a' as usize;
            if trie.nodes[index].is_none() {
                trie.nodes[index] = Some(Trie::new());
            }
            trie = trie.nodes[index].as_mut().unwrap();
        }
        trie.end = true;
    }

    /** Returns if the word is in the trie. */
    pub fn search(&mut self, word: String) -> bool {
        return self.find(word, true);
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    pub fn starts_with(&mut self, prefix: String) -> bool {
        return self.find(prefix, false);
    }

    fn find(&self, word: String, end: bool) -> bool {
        let mut trie = self;
        let cs: Vec<char> = word.chars().collect();
        for i in 0..cs.len() {
            let index = cs[i] as usize - 'a' as usize;
            if let Some(n) = &trie.nodes[index] {
                trie = n;
            } else {
                return false;
            }
        }
        if end {
            return trie.end;
        } else {
            return true;
        }
    }
}

#[test]
fn run() {
    let words = vec!["how", "hi", "her", "hello", "so", "see"];
    let t: Trie = Trie::create(words);

    let mut trie = Trie::new();
    trie.insert("apple".to_string());
    assert_eq!(trie.search("apple".to_string()), true);
    assert_eq!(trie.search("app".to_string()), false);
    assert_eq!(trie.starts_with("app".to_string()), true);
    trie.insert("app".to_string());
    assert_eq!(trie.search("app".to_string()), true);
}
