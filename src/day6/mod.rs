use std::collections::HashSet;
use std::io::BufRead;

use itertools::Itertools;

use crate::filesystem::get_file_handle;

fn get_formatted_data() -> Vec<Vec<char>>
{
    get_file_handle("./src/day6/input.txt").unwrap()
                                           .lines()
                                           .map(|l| l.unwrap().chars().collect_vec())
                                           .collect_vec()
}

fn find_guard_position(data: &[Vec<char>]) -> Option<(usize, usize)>
{
    const GUARD: char = '^';

    for (i, item) in data.iter().enumerate()
    {
        if let Some(j) = item.iter().position(|&x| x == GUARD)
        {
            return Some((i, j));
        }
    }
    None
}

fn solve_first_part() -> usize
{
    const OBSTRUCTION: char = '#';

    let mut move_stack = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle();
    let data = get_formatted_data();

    let mut guard_pos = find_guard_position(&data).unwrap();
    let mut move_pattern: &(i32, i32) = move_stack.next().unwrap();

    let mut seen = HashSet::new();
    seen.insert(guard_pos);

    loop
    {
        let new_c = (guard_pos.0 as i32) + move_pattern.0;
        let new_r = (guard_pos.1 as i32) + move_pattern.1;

        if (new_c < 0 || new_c > (data.len() - 1) as i32)
           || (new_r < 0 || new_r > (data[0].len() - 1) as i32)
        {
            // We will leave the room in next iteration.
            break;
        }
        if data[new_c as usize][new_r as usize] == OBSTRUCTION
        {
            // We do not move, just change the move pattern.
            move_pattern = move_stack.next().unwrap();
            continue;
        }
        guard_pos = (new_c as usize, new_r as usize);
        seen.insert(guard_pos);
    }
    seen.iter().count()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_part_solution()
    {
        let result = solve_first_part();

        assert_eq!(result, 4711);
    }
}
