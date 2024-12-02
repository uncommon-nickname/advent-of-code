use std::io::BufRead;

use itertools::Itertools;

use crate::filesystem::get_file_handle;

fn get_formatted_data() -> Vec<Vec<i32>>
{
    let handle = get_file_handle("./src/day2/input.txt").unwrap();

    handle.lines()
          .map(|l| l.unwrap().split_whitespace().map(|v| v.parse().unwrap()).collect_vec())
          .collect_vec()
}

fn solve_first_part() -> usize
{
    let data = get_formatted_data();

    data.iter()
        .filter(|v| {
            let sign = (v[1] - v[0]).signum();
            v.windows(2)
             .all(|w| (1..4).contains(&(w[1] - w[0]).abs()) && (w[1] - w[0]).signum() == sign)
        })
        .count()
}

fn solve_second_part() -> usize
{
    let data = get_formatted_data();

    data.iter()
        .filter(|v| {
            v.iter().combinations(v.len() - 1).any(|c| {
                                                  let sign = (c[1] - c[0]).signum();
                                                  c.windows(2).all(|w|
                                                                  (1..4).contains(&(w[1] - w[0]).abs())
                                                                  && (w[1] - w[0]).signum() == sign
                                                              )
                                              })
        })
        .count()
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_part_solution()
    {
        let result = solve_first_part();

        assert_eq!(result, 334);
    }

    #[test]
    fn test_second_part_solution()
    {
        let result = solve_second_part();

        assert_eq!(result, 400);
    }
}
