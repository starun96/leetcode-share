#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut scout_ptr = headd;
    while (scout_ptr.next.is_some()) {}
}
