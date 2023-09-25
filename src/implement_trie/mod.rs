use std::{cell::RefCell, rc::Rc, str::Chars};

type TrieNodeRef = Rc<RefCell<TrieNode>>;

struct TrieNode {
    val: char,
    children: Vec<TrieNodeRef>,
}

struct Trie {
    root: TrieNodeRef,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: Rc::new(RefCell::new(TrieNode {
                val: 'R',
                children: vec![],
            })),
        }
    }

    fn insert(&self, word: String) {
        /* for symbol in word.chars() {

        } */
    }

    fn search(&self, word: String) -> bool {
        todo!()
    }

    fn starts_with(&self, prefix: String) -> bool {
        todo!()
    }

    fn traverse(&self, word: String) -> impl Iterator {
        TrieIter {
            cur_node: Rc::clone(&self.root),
            word: word.chars().collect(),
            idx: 0,
        }
        .into_iter()
    }
}

struct TrieIter {
    cur_node: TrieNodeRef,
    word: Vec<char>,
    idx: usize,
}

impl Iterator for TrieIter {
    type Item = TrieNodeRef;

    fn next(&mut self) -> Option<Self::Item> {
        let cur_symbol = &self.word[self.idx];
        let x = self.cur_node.borrow().children.iter().find(|ch| todo!());
        todo!()
    }
}
