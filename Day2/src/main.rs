extern crate num_iter;

fn main() {
    let mut arr = [
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 5, 23, 1, 6, 23, 27,
        1, 27, 5, 31, 2, 1, 10, 0, 2, 35, 6, 39, 1, 39, 5, 43, 2, 43, 9, 1, 1, 47, 6, 51, 1, 13,
        51, 55, 2, 9, 55, 59, 1, 59, 13, 63, 1, 11, 63, 67, 2, 67, 10, 71, 1, 9, 71, 75, 2, 75, 6,
        79, 1, 79, 88, 83, 1, 83, 5, 87, 2, 9, 87, 91, 2, 9, 91, 95, 1, 95, 10, 99, 1, 9, 99, 103,
        2, 103, 6, 107, 2, 9, 107, 20, 1, 111, 5, 115, 2, 6, 115, 119, 1, 5, 119, 123, 1, 123, 2,
        127, 1, 127, 9, 0, 99, 2, 0, 8, 0,
    ];

    let sol = process_array(&mut arr);

    println!("{}", sol);
}

fn process_array(arr: &mut [i32]) -> i32 {
    let len = arr.len();

    for i in num_iter::range_step(0, len, 4) {
        let op = arr[i];
        let num1 = arr[i + 1];
        let num2 = arr[i + 2];
        if op == 99 {
            return arr[0];
        } else if op == 1 {
            arr[arr[i + 3] as usize] = num1 + num2;
        } else if op == 2 {
            arr[arr[i + 3] as usize] = num1 * num2;
        }
    }

    arr[0]
}
