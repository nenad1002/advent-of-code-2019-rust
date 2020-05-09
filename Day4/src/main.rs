fn main() {
    let start = 330;
    let end = 343411;

    let mut count = 0;

    for i in start..end {
        count += if is_valid(i) { 1 } else { 0 };
    }

    println!("{}", count);
}

fn is_valid(num: i32) -> bool {
    let mut n = num;
    let mut prev = 10;
    let mut is_there_double: bool = false;
    while n > 0 {
        let digit = n % 10;
        if digit > prev {
            return false;
        }
        if digit == prev {
            is_there_double = true;
        }

        prev = digit;
        n /= 10;
    }

    return is_there_double;
}
