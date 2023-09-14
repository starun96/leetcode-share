use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct HeapNode {
    elem: i32,
    count: u32,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.count.cmp(&self.count)
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.count.partial_cmp(&self.count)
    }
}

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut count_map: HashMap<i32, u32> = HashMap::new();
    for num in nums {
        *count_map.entry(num).or_default() += 1;
    }

    let mut bin_heap = BinaryHeap::new(); // max heap

    for (elem, count) in count_map.iter().take(k) {
        bin_heap.push(HeapNode {
            elem: elem.clone(),
            count: count.clone(),
        });
    }

    for (elem, count) in count_map.into_iter().skip(k) {
        let min_elem = bin_heap.peek().unwrap();
        if count > min_elem.count {
            bin_heap.pop();
            bin_heap.push(HeapNode { elem, count });
        }
    }

    bin_heap.into_iter().map(|node| node.elem).collect()
}
