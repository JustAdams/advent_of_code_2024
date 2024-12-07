use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe_count = 0;

        for line in reader.lines() {
            let mut is_safe = true;

            let line = line?;
            let line_vec = line.split_whitespace().collect::<Vec<&str>>();

            let mut prev_num = line_vec[0].parse::<i32>()?;
            let is_increasing = prev_num < line_vec[1].parse::<i32>()?;

            for i in 1..line_vec.len() {
                let curr_num = line_vec[i].parse::<i32>()?;
                if (is_increasing && prev_num >= curr_num) || (!is_increasing && prev_num <= curr_num) {
                    is_safe = false;
                    break;
                } else if (prev_num - curr_num).abs() > 3 {
                    is_safe = false;
                    break;
                }
                prev_num = curr_num;
            }

            if is_safe {
                 safe_count += 1;
            }
        }
        Ok(safe_count)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
