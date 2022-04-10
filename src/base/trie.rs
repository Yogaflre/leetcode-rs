// <字典树>
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Trie {
    pub nodes: Vec<Option<Trie>>,
    pub end: bool, //从根节点开始是否为完整单词
}
