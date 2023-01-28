use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

pub trait RemovePercent<T> {
    fn remove_percent<P>(self, percent: usize, predicate: P) -> Self
    where
        P: Fn(&T) -> bool;
}

impl<T> RemovePercent<T> for Vec<T> {
    fn remove_percent<P>(self, percent: usize, predicate: P) -> Self
    where
        P: Fn(&T) -> bool,
    {
        let mut removed = 0;
        let vec_len = self.len();

        self.into_iter()
            .filter(|i| {
                if (100 * removed) / vec_len > percent {
                    true
                } else if predicate(&i) {
                    removed += 1;

                    false
                } else {
                    true
                }
            })
            .collect()
    }
}

pub fn most_frequent<T>(array: &[T], k: usize) -> Vec<(usize, &T)>
where
    T: Hash + Eq + Ord,
{
    let mut map = HashMap::with_capacity(array.len());
    for x in array {
        *map.entry(x).or_default() += 1;
    }

    let mut heap = BinaryHeap::with_capacity(k + 1);
    for (x, count) in map.into_iter() {
        if heap.len() < k {
            heap.push(Reverse((count, x)));
        } else {
            let &Reverse((min, _)) = heap.peek().unwrap();
            if min < count {
                heap.pop();
                heap.push(Reverse((count, x)));
            }
        }
    }
    heap.into_sorted_vec().into_iter().map(|r| r.0).collect()
}

#[test]
fn remove_20_percent() {
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(
        vec.remove_percent(20, |i| i % 2 == 0),
        vec![1, 3, 5, 7, 8, 9, 10]
    );
}
