use std::collections::HashMap;
use std::fs;

struct ExtractedInput {
    left_vec: Vec<i32>,
    right_vec: Vec<i32>,
}

fn main() {
    part_1();
    part_2();
}

fn extract_input_vecs() -> ExtractedInput {
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

    ExtractedInput {
        left_vec,
        right_vec
    }
}

fn part_1() {
    println!("Part 1 start");

    let extracted = extract_input_vecs();

    let mut left_vec = extracted.left_vec;
    let mut right_vec = extracted.right_vec;

    left_vec.sort_by(|a, b| a.cmp(b));
    right_vec.sort_by(|a, b| a.cmp(b));

    let mut total_distance = 0;
    for i in 0..left_vec.len() {
        total_distance += (left_vec[i] - right_vec[i]).abs();
    }

    println!("Total Distance: {}", total_distance);
    println!("Part 1 end");
}

/** Add up each number in the left list after multiplying
it by the number of times it appears in the right list. */
fn part_2() {

    let extracted = extract_input_vecs();

    let mut left_vec = extracted.left_vec;
    let mut right_vec = extracted.right_vec;

    // build count of occurrences in right list
    let mut count_map = HashMap::new();
    for right_num in right_vec.iter() {
        let count = count_map.entry(right_num).or_insert(0);
        *count += 1;
    }

    let mut mult_distance = 0;
    for left_num in left_vec.iter() {
        mult_distance += left_num * count_map.get(left_num).unwrap_or(&0);
    }

    println!("Total Distance: {}", mult_distance);
}