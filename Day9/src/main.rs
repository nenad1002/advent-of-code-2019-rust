use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn biggest_elements(arr: &[i32], n: usize) -> Vec<i32> {
    let mut heap = BinaryHeap::with_capacity(n);

    for item in arr {
        heap.push(Reverse(item));
        if heap.len() > n {
            heap.pop();
        }
    }

    let mut res = vec![1; n];
    let mut i = (n - 1) as isize;

    while !heap.is_empty() {
        if let Some(el) = heap.peek() {
            //println!("{:?}", el);
            res[i as usize] = el.0.clone();
            i -= 1;
        }
        heap.pop();
    }

    res
}

fn main() {
    let el = [2, 3, 1, 0, -1, 4, 10, -2];

    println!("{:?}", biggest_elements(&el, 5));
}
