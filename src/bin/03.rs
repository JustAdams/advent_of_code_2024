use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;


const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<()> {
    start_day(DAY);


    //region Part 1
    println!("=== Part 1 ===");

    /** Add up the product of each instruction */
    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut answer = 0;
        let mul_regex: Regex = Regex::new("mul[(][0-9]{1,3},{1}[0-9]{1,3}[)]").unwrap();
        let num_regex: Regex = Regex::new("[0-9]{1,3}").unwrap();

        for line in reader.lines() {
            let line = line?;
            // create vec of all mul patterns
            let mul_vec: Vec<&str> = mul_regex.find_iter(&*line).map(|m| m.as_str()).collect();

            // parse out the numbers in each mul instruction and sum the products
            for needle in mul_vec {
                let mut curr_num = 1;
                let nums: Vec<&str> = num_regex.find_iter(needle).map(|m| m.as_str()).collect();
                for num in nums {
                    curr_num *= num.parse::<i32>()?;
                }
                answer += curr_num;
            }
        }

        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let mut answer = 0;
        let mul_regex: Regex = Regex::new(r"mul[(][0-9]{1,3},{1}[0-9]{1,3}[)]").unwrap();
        let num_regex: Regex = Regex::new("[0-9]{1,3}").unwrap();
        let do_regex: Regex = Regex::new(r"do\(\)").unwrap();
        let dont_regex: Regex = Regex::new(r"don't\(\)").unwrap();

        let mut do_mul: bool = true;

        for line in reader.lines() {
            let line = line?;
            // create vec of all mul patterns
            let mul_vec: Vec<&str> = mul_regex.find_iter(&*line).map(|m| m.as_str()).collect();
            let do_vec: Vec<&str> = do_regex.find_iter(&*line).map(|m| m.as_str()).collect();
            let dont_vec: Vec<&str> = dont_regex.find_iter(&*line).map(|m| m.as_str()).collect();

            println!("{}", line);
            // find all mul instructions / indices
            let mul_idx: Vec<_> = line.match_indices(&mul_regex.as_str()).collect::<Vec<_>>();

            for (i, val) in mul_idx {
                println!("{}, {}", i, val);
            }

            // // parse out the numbers in each mul instruction and sum the products
            // if do_mul {
            //     for needle in mul_vec {
            //         let mut curr_num = 1;
            //         let nums: Vec<&str> = num_regex.find_iter(needle).map(|m| m.as_str()).collect();
            //         for num in nums {
            //             curr_num *= num.parse::<i32>()?;
            //         }
            //         answer += curr_num;
            //     }
            // }
        }

        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST_2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
