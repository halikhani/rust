use std::cmp::Reverse; // wrapsa a value and flips its ordering
use std::collections::BinaryHeap;
use ordered_float::OrderedFloat;

pub fn top_k_sort(logits: &[f32], k: usize) -> Vec<(usize, f32)> {
    let mut indexed: Vec<_> = logits.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    // iter() iterates over it, enumerate() adds positions (0, &f32), (1, &f32), etc.
    // |(i, &v)| (i, v) destructures the reference to get owned values (i, v)
    // collect() turns the iterator into a vector
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // partial_cmp() compares f32 values, returns Option<Ordering>
    indexed.truncate(k);
    indexed
}

pub fn top_k_heap(logits: &[f32], k: usize) -> Vec<(usize, f32)> {
    if k == 0 || logits.is_empty() {
        return Vec::new();
    }
    let k = k.min(logits.len());

    let mut heap: BinaryHeap::<Reverse<(OrderedFloat<f32>, usize)>> = BinaryHeap::with_capacity(k); //?
    for (i, &value) in logits.iter().enumerate() {
        let candidate = (OrderedFloat(value), i);
        if heap.len() < k {
            heap.push(Reverse(candidate));
        }
        else {
            if *heap.peek().unwrap() > Reverse(candidate) {
                heap.pop();
                heap.push(Reverse(candidate));
            }
        }
    }

    let mut out: Vec<(usize, f32)> = heap
        .into_iter()
        .map(|Reverse((s, i))| (i, s.into_inner()))
        .collect();

        out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        out
}