use std::collections::HashMap;

fn main() {
    let arr1 = ["R75", "D33", "R83", "U81", "L12", "D49", "R71", "U7", "L72"];
    let arr2 = ["U62", "R66", "U55", "R20", "D72", "R15", "D58", "R83"];

    println!("{}", calculate_distance(&arr1, &arr2));
}

fn calculate_distance(arr1: &[&str], arr2: &[&str]) -> i32 {
    let mut points = HashMap::new();

    let mut position = (0, 0);

    let mut result = std::i32::MAX;
    for item in arr1.iter() {
        let direction = &item[0..1];
        let walk = get_walk(&item);
        let direction_num = get_numerical_direction(&direction);

        position.0 += direction_num.0 * walk;
        position.1 += direction_num.1 * walk;
        for i in position.0 + 1..(position.0 + direction_num.0 * walk + 1) {
            points.insert((i, position.1), true);
        }

        for i in position.1 + 1..(position.1 + direction_num.1 * walk + 1) {
            points.insert((position.0, i), true);
        }
    }

    for item in arr2.iter() {
        let direction = &item[0..1];
        let walk = get_walk(&item);

        let direction_num = get_numerical_direction(&direction);

        position.0 += direction_num.0 * walk;
        position.1 += direction_num.1 * walk;

        for i in position.0 + 1..(position.0 + direction_num.0 * walk + 1) {
            let pos = (i, position.1);
            if points.contains_key(&pos) {
                let distance = pos.0 * pos.0 + pos.1 * pos.1;
                result = std::cmp::min(result, distance);
            }
        }

        for i in position.1 + 1..(position.1 + direction_num.1 * walk + 1) {
            let pos = (position.0, i);
            if points.contains_key(&pos) {
                let distance = pos.0 * pos.0 + pos.1 * pos.1;
                result = std::cmp::min(result, distance);
            }
        }
    }

    return (result as f64).sqrt() as i32;
}

fn get_walk(item: &str) -> i32 {
    return item[1..].parse::<i32>().unwrap();
}

fn get_numerical_direction(direction: &str) -> (i32, i32) {
    match direction {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, -1),
        "D" => (0, 1),
        _ => (0, 0),
    }
}
