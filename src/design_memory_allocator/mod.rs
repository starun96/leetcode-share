

use std::ops::Range;

use std::collections::LinkedList;

struct Allocator {
    mem_arr: Vec<usize>,
    capacity: usize,
    free_ranges: LinkedList<Range<usize>>, // points into `mem_arr`
}

impl Allocator {
    fn new(n: usize) -> Self {
        let mut free_ranges = LinkedList::new();
        free_ranges.push_front(0..n);
        Self {
            mem_arr: vec![0; n],
            capacity: n,
            free_ranges,
        }
    }

    fn allocate(&self, size: i32, m_id: i32) -> i32 {
        let size = size as usize;
        let m_id = m_id as usize;

        if size > self.capacity {
            return -1;
        }

        if size == 0 {
            panic!("Can't allocate 0 blocks")
        }

        // not necessarily true that we even have N contiguous slots
        // i.e. fragmentation might mess us up

        // current working thought is to model after malloc(1)
        // i.e. start with free list of extents
        // base / initial extent is covering the whole array, single node

        // allocations carve slices out of the extents
        // i.e. first time will shorten extent from front
        // next allocation would further shorten this
        // subsequent free() may then re-open entries at beginning
        // hence the free list needs 2 nodes to represent the gap between!

        // (funnily enough you can actually have a 0 sized gap
        // unless you merge free list blocks)

        // find first slab that's AT LEAST as big as the allocation requirement
        let Some((idx, &first_free_range)) = self
            .free_ranges
            .iter()
            .enumerate()
            .find(|(idx, range)| range.len() >= size)
        else {
            return -1;
        };

        // that's the key thing, might have an allocation candidate that's
        // far bigger than we really need
        // best practice is to split the allocation
        // splitting the allocation then amounts to cutting the region into 2 parts
        // (1) the part we need (2) the extra stuff
        // we connect the node PRECEDING (1) to (2)
        // so we 'jump' over the allocation we just sliced out.

        let range_len = first_free_range.len();
        let Range { start , end } = first_free_range;

        // great, we needed all of this memory anyway.
        // delete the node from the free list.
        if size as usize == range_len {
            self.mem_arr[start..start + size].fill(m_id);
            self.free_ranges.remove(idx);
        } else {
            // time to split the allocation
            let extra = range_len - size;
            let prev = first_free_range.previous()
            prev.connectTo(extra)

        }
        todo!()
    }

    fn free(&self, m_id: i32) -> i32 {}
}
