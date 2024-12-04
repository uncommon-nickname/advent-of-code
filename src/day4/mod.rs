use std::io::BufRead;

use itertools::Itertools;

use crate::filesystem::get_file_handle;

fn get_formatted_data() -> Vec<Vec<char>>
{
    get_file_handle("./src/day4/input.txt").unwrap()
                                           .lines()
                                           .map(|l| l.unwrap().chars().collect_vec())
                                           .collect_vec()
}

fn solve_first_part() -> usize
{
    const LETTERS: [char; 4] = ['X', 'M', 'A', 'S'];
    const DIRS: [(i32, i32); 8] =
        [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];

    let data = get_formatted_data();

    let cols = data.len();
    let rows = data[0].len();

    let mut count = 0;
    for c in 0..cols
    {
        for r in 0..rows
        {
            if data[c][r] != LETTERS[0]
            {
                continue;
            }
            for dir in DIRS.iter()
            {
                let mut was_matched = true;

                for (k, letter) in LETTERS.iter().enumerate()
                {
                    let new_c = c as i32 + dir.0 * k as i32;
                    let new_r = r as i32 + dir.1 * k as i32;

                    if (new_c < 0 || new_c >= cols as i32)
                       || (new_r < 0 || new_r >= rows as i32)
                       || (data[new_c as usize][new_r as usize] != *letter)
                    {
                        was_matched = false;
                        break;
                    }
                }
                if was_matched
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve_second_part() -> usize
{
    let data = get_formatted_data();

    let cols = data.len();
    let rows = data[0].len();

    let mut count = 0;
    for c in 1..cols - 1
    {
        for r in 1..rows - 1
        {
            if data[c][r] != 'A'
            {
                continue;
            }

            if ((data[c - 1][r - 1] == 'M' && data[c + 1][r + 1] == 'S')
                || (data[c - 1][r - 1] == 'S' && data[c + 1][r + 1] == 'M'))
               && ((data[c - 1][r + 1] == 'M' && data[c + 1][r - 1] == 'S')
                   || (data[c - 1][r + 1] == 'S' && data[c + 1][r - 1] == 'M'))
            {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_part_solution()
    {
        let result = solve_first_part();

        assert_eq!(result, 2633);
    }

    #[test]
    fn test_second_part_solution()
    {
        let result = solve_second_part();

        assert_eq!(result, 1936);
    }
}
