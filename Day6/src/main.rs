use std::collections::HashMap;

struct SparseVector {
    index_map: HashMap<i32, i32>,
}

impl SparseVector {
    fn new() -> SparseVector {
        return SparseVector {
            index_map: HashMap::new(),
        };
    }

    fn hydrate(&mut self, nums: Vec<i32>) {
        for i in 0..nums.len() {
            if nums[i] != 0 {
                self.index_map.insert(i as i32, nums[i]);
            }
        }
    }

    fn dot_product(&self, vec: SparseVector) -> i32 {
        let mut product = 0;
        for (key, value) in self.index_map.clone().into_iter() {
            if vec.index_map.contains_key(&key) {
                product += value * vec.index_map[&key];
            }
        }
        return product;
    }
}
fn main() {
    let vec1 = vec![1, 3, 0, 2, 1];
    let vec2 = vec![0, 3, 0, 4, 0];

    let mut spv1 = SparseVector::new();
    let mut spv2 = SparseVector::new();

    spv1.hydrate(vec1);
    spv2.hydrate(vec2);

    println!("{}", spv1.dot_product(spv2));
}
