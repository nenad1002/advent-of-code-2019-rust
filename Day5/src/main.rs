use std::cmp;

fn main() {
    let arr = [1, 2, 3, 4];
    let start = 0;
    let dest = 3;

    println!("{}", calculate(&arr, start, dest));
}

fn calculate(arr: &[usize], start: usize, dest: usize) -> usize {
    let mut sum = 0;
    let mut res = 0;
    for i in 0..arr.len() {
        if i >= start && i < dest {
            sum += arr[i];
        }
        res += arr[i];
    }
    return cmp::min(sum, res - sum);
}
