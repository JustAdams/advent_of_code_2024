use std::fs;

fn main() {
    part_1();
}

fn part_1() {
    println!("Part 1 start");

    // iterate over each input line and take the left/right values after converting
    // to an i32
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    for line in contents.lines() {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        left_vec.push(split_line[0].parse::<i32>().unwrap());
        right_vec.push(split_line[1].parse::<i32>().unwrap());
    }

    left_vec.sort_by(|a, b| a.cmp(b));
    right_vec.sort_by(|a, b| a.cmp(b));

    let mut total_distance = 0;
    for i in 0..left_vec.len() {
        total_distance += (left_vec[i] - right_vec[i]).abs();
    }

    println!("Total Distance: {}", total_distance);
    println!("Part 1 end");
}