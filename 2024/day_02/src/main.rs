use std::fs;

fn main() {
    // Read input data
    let file = fs::read_to_string("src/input.txt").expect("Unable to read file");
    // Read datafile
    let table = parse_data(&file);
    // Run task one
    println!("Number of rows that are safe: {}", task_one(table));
}

fn task_one(input: impl Iterator<Item = Vec<i32>>) -> i32 {
    let mut result: i32 = 0;
    for row in input {
        let checked_row = row.iter().zip(row.iter().skip(1)).map(|(current, next)| {
            if current - next <= 3 && current - next > 0 {
                return 1;
            } else if current - next >= -3 && current - next < 0 {
                return -1;
            } else {
                0
            }
        });
        let distance_sum: i32 = checked_row.clone().sum();
        let distance_sum_lenght = checked_row.count() as i32;

        if distance_sum - distance_sum_lenght == 0 || distance_sum + distance_sum_lenght == 0 {
            result += 1;
        }
    }
    return result;
}

// Split data per row and parse it, this function can also sort the data
fn parse_data(data: &String) -> impl Iterator<Item = Vec<i32>> + use<'_> {
    let data_rows = data.lines();
    let parsed_data_rows = data_rows.map(|row| create_vec(row));
    return parsed_data_rows;
}

// Parses string into vector of integers
fn create_vec(row: &str) -> Vec<i32> {
    let parsed_data: Vec<i32> = row
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    return parsed_data;
}
