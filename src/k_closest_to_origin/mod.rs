use std::collections::BinaryHeap;

fn heap_logic(
    mut heap: BinaryHeap<(i32, usize)>,
    idx: usize,
    x: i32,
    y: i32,
    k: usize,
) -> BinaryHeap<(i32, usize)> {
    let squared_distance = x * x + y * y;
    heap.push((squared_distance, idx));
    if heap.len() > k {
        heap.pop();
    }
    heap
}

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    points
        .iter()
        .enumerate()
        .fold(BinaryHeap::new(), |heap, (idx, p)| {
            heap_logic(heap, idx, p[0], p[1], k as usize)
        })
        .into_iter()
        .map(|(_, idx)| points[idx].clone())
        .collect()
}
