use std::io::Read;

use itertools::Itertools;
use regex::Regex;

use crate::filesystem::get_file_handle;

fn solve_first_part() -> usize
{
    let mut buf = String::new();
    get_file_handle("./src/day3/input.txt").unwrap().read_to_string(&mut buf).unwrap();

    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();

    re.captures_iter(&buf)
      .map(|c| c.extract::<2>().1)
      .map(|v| v[0].parse::<usize>().unwrap() * v[1].parse::<usize>().unwrap())
      .sum::<usize>()
}

fn solve_second_part() -> usize
{
    let mut buf = String::new();
    get_file_handle("./src/day3/input.txt").unwrap().read_to_string(&mut buf).unwrap();

    let re = Regex::new(r"(?:mul\((?<first>\d+),(?<second>\d+)\)|do\(\)(?<a>)(?<b>)|don't\(\)(?<c>)(?<d>))").unwrap();
    let matches = re.captures_iter(&buf).map(|c| c.extract::<2>()).collect_vec();

    let mut result = 0;
    let mut should_add = true;

    for m in matches.iter()
    {
        if m.0 == "do()"
        {
            should_add = true;
            continue;
        }
        if m.0 == "don't()"
        {
            should_add = false;
            continue;
        }
        if !should_add
        {
            continue;
        }
        result += m.1[0].parse::<usize>().unwrap() * m.1[1].parse::<usize>().unwrap();
    }
    result
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_part_solution()
    {
        let result = solve_first_part();

        assert_eq!(result, 181_345_830);
    }

    #[test]
    fn test_second_part_solution()
    {
        let result = solve_second_part();

        assert_eq!(result, 98_729_041);
    }
}
