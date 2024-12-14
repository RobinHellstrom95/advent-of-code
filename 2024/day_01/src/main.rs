use std::fs;

fn main() {
    // Read input data
    let data = fs::read_to_string("./src/input.txt").expect("Unable to read file");

    // Parse first column and sort it
    let column_1 = parse_data(data.clone(), 0, true);
    let column_2 = parse_data(data.clone(), 1, true);

    part_one(column_1.clone(), column_2.clone());
    part_two(column_1.clone(), column_2.clone());
}

fn part_two(column_1: Vec<i32>, column_2: Vec<i32>) {
    let distance = column_1.iter().map(|x| test_fun(*x, column_2.clone()));
    println!("Summation of the distance for part two!");
    let distance_sum: i32 = distance.sum();
    println!("{}", distance_sum);
}

fn test_fun(row: i32, column: Vec<i32>) -> i32 {
    let mut iter_value = 0;
    for value in column {
        if value == row {
            iter_value += row;
        }
    }
    return iter_value;
}

fn part_one(column_1: Vec<i32>, column_2: Vec<i32>) {
    let distance = column_1
        .iter()
        .zip(column_2.iter())
        .map(|(x, y)| if x >= y { x - y } else { y - x });

    println!("Summation of the distance for part one!");
    let distance_sum: i32 = distance.sum();
    println!("{}", distance_sum);
}

// Split data and parse it, this function can also sort the data
fn parse_data(data: String, column: usize, sort: bool) -> Vec<i32> {
    let mut parsed_data: Vec<i32> = data
        .split_whitespace()
        .skip(column)
        .step_by(2)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    if sort {
        parsed_data.sort();
    }

    return parsed_data;
}
