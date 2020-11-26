use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::BufRead;
use std::io::{prelude::*, BufReader};
use std::path::Path;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    file_num: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.cost.cmp(&other.cost))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

fn main() -> std::io::Result<()> {
    let f1 = File::open("./list1.in")?;
    let bf1 = BufReader::new(f1);
    let f2 = File::open("./list2.in")?;
    let bf2 = BufReader::new(f2);

    let mut arr = [bf1, bf2];

    let n = 5;

    let mut heap = BinaryHeap::with_capacity(n);

    let mut i = 0;

    for item in &mut arr {
        let mut s = String::new();
        item.read_line(&mut s);
        println!("{}", s);
        let item = State {
            cost: s.trim().clone().parse::<i32>().unwrap(),
            file_num: i,
        };
        heap.push(item);
        if heap.len() > n {
            heap.pop();
        }
        i += 1;
    }

    let mut res = vec![1; n];
    //let mut i = (n - 1) as isize;

    while !heap.is_empty() {
        if let Some(el) = heap.peek() {
            let file_num = el.file_num;
            let mut new_cost = String::new();
            let num_bytes = arr[file_num].read_line(&mut new_cost).unwrap();
            if num_bytes != 0 {
                println!("{}", new_cost);
                heap.push(State {
                    cost: new_cost.trim().clone().parse::<i32>().unwrap(),
                    file_num: file_num,
                })
            }
        }
        heap.pop();
    }

    println!("{:?}", res);

    Ok(())
}
