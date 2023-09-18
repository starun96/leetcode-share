use std::{cell::RefCell, collections::HashMap, ops::Deref, rc::Rc};

type NodeRef = Rc<RefCell<Node>>;

struct Node {
    pub key: i32,
    pub val: i32,
    pub next: Option<NodeRef>,
}

pub struct LRUCache {
    head: NodeRef,
    tail: NodeRef,
    node_map: HashMap<i32, NodeRef>,
    capacity: usize,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let head_node = Rc::new(RefCell::new(Node {
            key: -1,
            val: 0,
            next: None,
        }));
        LRUCache {
            head: Rc::clone(&head_node),
            tail: Rc::clone(&head_node),
            node_map: HashMap::from([(-1, Rc::clone(&head_node))]),
            capacity: capacity as usize,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.pop(key) {
            Some(target_node_ref) => {
                let value = target_node_ref.borrow().val;
                self.append(target_node_ref);
                value
            }
            None => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let target_node_ref = self.pop(key);
        match target_node_ref {
            Some(target_node_inner) => {
                target_node_inner.borrow_mut().val = value;
                self.append(target_node_inner);
            }
            None => {
                let new_node = Rc::new(RefCell::new(Node {
                    key,
                    val: value,
                    next: None,
                }));

                self.append(new_node);

                if self.node_map.len() > self.capacity {
                    let first_node_key = self.head.borrow().next.as_ref().unwrap().borrow().key;
                    self.pop(first_node_key);
                }
            }
        }
    }

    fn pop(&mut self, key: i32) -> Option<NodeRef> {
        let prev_node = self.node_map.get(&key)?;

        // clone prev
        let target_node = Rc::clone(prev_node.borrow().next.as_ref().unwrap());

        // &(prev.next)
        let target_node_borrow = target_node.borrow();
        let next_node = target_node_borrow.next.as_ref();

        if let Some(next_node_inner) = next_node {
            prev_node.deref().borrow_mut().next = Some(Rc::clone(next_node_inner));
            self.node_map
                .insert(next_node_inner.borrow().key, Rc::clone(prev_node));
        }
        drop(target_node_borrow);
        target_node.deref().borrow_mut().next = None;
        self.node_map.remove(&key);

        return Some(target_node);
    }

    fn append(&mut self, node_ref: NodeRef) {
        let tail = Rc::clone(&self.tail);
        let key = node_ref.borrow().key;
        tail.deref().borrow_mut().next = Some(Rc::clone(&node_ref));
        self.node_map.insert(key, tail);
        self.tail = node_ref;
    }
}

struct LRUCacheIter {
    cur_node: NodeRef,
}

impl Iterator for LRUCacheIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        let current_node_borrow = self.cur_node.borrow();
        let next_node = &current_node_borrow.next?;
        let next_node_borrow = next_node.borrow();
        //drop(current_node_borrow);
        Some((next_node_borrow.key, next_node_borrow.val))
    }
}
