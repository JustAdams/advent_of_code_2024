use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

struct ExtractedInput {
    left_vec: Vec<i32>,
    right_vec: Vec<i32>,
}

fn extract_input_vecs<R: BufRead>(reader: R) -> ExtractedInput {
    // iterate over each input line and take the left/right values after converting
    // to an i32

    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    for line in reader.lines() {
        let unwrapped_line = line.unwrap();
        let split_line = unwrapped_line.split_whitespace().collect::<Vec<&str>>();
        left_vec.push(split_line[0].parse::<i32>().unwrap());
        right_vec.push(split_line[1].parse::<i32>().unwrap());
    }

    ExtractedInput {
        left_vec,
        right_vec
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let extracted_input = extract_input_vecs(reader);

        let mut left_vec = extracted_input.left_vec;
        let mut right_vec = extracted_input.right_vec;

        left_vec.sort_by(|a, b| a.cmp(b));
        right_vec.sort_by(|a, b| a.cmp(b));

        let mut total_distance = 0;
        for i in 0..left_vec.len() {
            total_distance += (left_vec[i] - right_vec[i]).abs();
        }


        Ok(total_distance)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let extracted_input = extract_input_vecs(reader);
        let mut left_vec = extracted_input.left_vec;
        let mut right_vec = extracted_input.right_vec;

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
        Ok(mult_distance)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
