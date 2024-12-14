use std::fs;

fn main() {
    // Read input data
    let data: String = fs::read_to_string("./src/input.txt").expect("Unable to read file");

    // Parse first column and sort it
    let column_1 = parse_data(data.clone(), 0, true);
    let column_2 = parse_data(data.clone(), 1, true);

    let distance = column_1
        .iter()
        .zip(column_2.iter())
        .map(|(x, y)| if x >= y { x - y } else { y - x });

    println!("Summation of the distance");
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
