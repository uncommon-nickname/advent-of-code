use std::io::BufRead;

use itertools::Itertools;

use crate::filesystem::get_file_handle;

fn get_formatted_data() -> (Vec<i32>, Vec<i32>)
{
    const INPUT_SIZE: usize = 1_000;

    let handle = get_file_handle("./src/day1/input.txt").unwrap();

    let mut first = Vec::with_capacity(INPUT_SIZE);
    let mut second = Vec::with_capacity(INPUT_SIZE);

    for line in handle.lines()
    {
        let values = line.unwrap().split_whitespace().map(|v| v.parse().unwrap()).collect_vec();

        first.push(values[0]);
        second.push(values[1]);
    }
    (first, second)
}

fn solve_first_part() -> i32
{
    let (mut first, mut second) = get_formatted_data();

    first.sort();
    second.sort();

    first.iter().zip(second.iter()).map(|(v1, v2)| (*v1 - *v2).abs()).sum()
}

fn solve_second_part() -> i32
{
    let (first, second) = get_formatted_data();

    first.iter().map(|v| *v * second.iter().filter(|x| *x == v).count() as i32).sum()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_part_solution()
    {
        let result = solve_first_part();

        assert_eq!(result, 2_580_760);
    }

    #[test]
    fn test_second_part_solution()
    {
        let result = solve_second_part();

        assert_eq!(result, 25_358_365);
    }
}
