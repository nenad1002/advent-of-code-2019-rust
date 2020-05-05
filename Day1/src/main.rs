fn main() {
    let arr = [12, 14];

    println!("{}", calculate_result(&arr));
}

fn calculate_result(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for item in arr.iter() {
        sum += item / 3 - 2;
    }

    sum
}
