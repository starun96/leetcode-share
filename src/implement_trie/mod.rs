use std::{cell::RefCell, rc::Rc, str::Chars};

type TrieNodeRef = Rc<RefCell<TrieNode>>;

struct TrieNode {
    pub val: char,
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
        let traverser = self.traverse(word).enumerate();
        let (last_idx, last_node) = traverser.last().unwrap();

        let mut current_node = Some(last_node);
        let mut cur_idx = last_idx;

        while let Some(current_node_ref) = current_node {
            let cur_char = &word.chars().nth(cur_idx).unwrap();
            let new_node = Rc::new(RefCell::new(TrieNode {
                val: *cur_char,
                children: vec![],
            }));
            let current_node_ref_borrow = current_node_ref.borrow();
            current_node_ref_borrow.children.push(new_node);
        }
    }

    fn search(&self, word: String) -> bool {
        todo!()
    }

    fn starts_with(&self, prefix: String) -> bool {
        todo!()
    }

    fn traverse(&self, word: String) -> TrieIter {
        TrieIter {
            cur_node: Rc::clone(&self.root),
            word: word.chars().collect(),
            idx: 0,
        }
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
        let cur_symbol = self.word.get(self.idx)?;
        let borrowed_node = self.cur_node.borrow();
        let next_child = borrowed_node
            .children
            .iter()
            .find(|child| child.borrow().val == *cur_symbol)?;
        let new_node = Rc::clone(next_child);
        drop(borrowed_node);
        self.cur_node = new_node;
        self.idx += 1;

        Some(Rc::clone(&self.cur_node))
    }
}
