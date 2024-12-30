use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
....b.....
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    /** Converts the input BufRead into a 2D vec of type char */
    fn convert_to_map<R: BufRead>(reader: R) -> Result<Vec<Vec<char>>, Error> {
        let mut map: Vec<Vec<char>> = Vec::new();
        // convert input string into 2D array
        for line in reader.lines() {
            let char_line: Vec<char> = line?.chars().collect();
            map.push(char_line);
        }
        Ok(map)
    }


    /** If the next position is out of bounds, return true  */
    fn next_move_ends(map: &Vec<Vec<char>>, next_pos: [i32; 2]) -> bool {
        let next_row = next_pos[0];
        let next_col = next_pos[1];
        if next_row < 0 || next_col < 0 { return true; }
        if next_row >= map.len() as i32 || next_col >= map[0].len() as i32 { return true; }
        false
    }

    //region Part 1
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let mut answer: i32 = 0;
        let mut curr_pos = [0, 0];
        let mut curr_dir_idx = 0;

        let mut map = convert_to_map(reader)?;
        let height = map.len();
        let width = map[0].len();
        // find starting position
        for row in 0..height {
            for col in 0..width {
                if map[row][col] == '^' {
                    curr_pos = [row, col];
                    break;
                }
            }
        }

        if next_move_ends(&map, [(curr_pos[0] as i32) - 1, curr_pos[1] as i32]) {
            return Ok(answer);
        }

        let mut next_pos: [usize; 2] = [(curr_pos[0] - 1), curr_pos[1]];
        loop {
            if map[curr_pos[0]][curr_pos[1]] != 'X' {
                answer += 1;
            }
            // mark current position as visited
            map[curr_pos[0]][curr_pos[1]] = 'X';

            // set the next position in a gross way that I "totally" will improve later
            let mut next_row: i32 = curr_pos[0] as i32;
            let mut next_col: i32 = curr_pos[1] as i32;
            match curr_dir_idx {
                0 => next_row -= 1,
                1 => next_col += 1,
                2 => next_row += 1,
                3 => next_col -= 1,
                _ => unreachable!()
            }

            if next_move_ends(&map, [next_row, next_col]) {
                break;
            }
            next_pos = [next_row as usize, next_col as usize];

            // rotate 90* if reaching a wall, otherwise move forward
            if map[next_pos[0]][next_pos[1]] == '#' {
                curr_dir_idx = (curr_dir_idx + 1) % 4;
            } else {
                curr_pos = next_pos;
            }
        }

        Ok(answer)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

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
