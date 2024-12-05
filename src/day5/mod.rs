use std::collections::{HashMap, HashSet};
use std::io::BufRead;

use itertools::{Either, Itertools};

use crate::filesystem::get_file_handle;

fn get_formatted_data() -> (Vec<Vec<usize>>, HashMap<usize, Vec<usize>>)
{
    let (r, d): (Vec<String>, Vec<String>) =
        get_file_handle("./src/day5/input.txt").unwrap()
                                               .lines()
                                               .map(|l| l.unwrap())
                                               .filter(|l| !l.is_empty())
                                               .partition_map(|l| {
                                                   if l.contains("|")
                                                   {
                                                       Either::Left(l)
                                                   }
                                                   else
                                                   {
                                                       Either::Right(l)
                                                   }
                                               });

    let mut rules = HashMap::new();

    r.iter()
     .map(|l| l.split("|").map(|v| v.parse::<usize>().unwrap()).collect_tuple().unwrap())
     .for_each(|(l, r)| rules.entry(l).or_insert_with(Vec::new).push(r));

    let update = d.iter()
                  .map(|l| l.split(",").map(|v| v.parse::<usize>().unwrap()).collect_vec())
                  .collect_vec();

    (update, rules)
}

fn solve_first_part() -> usize
{
    let (update, rules) = get_formatted_data();

    let mut sum = 0;
    for single in update.iter()
    {
        let mut seen = HashSet::new();
        let mut valid = true;

        for item in single.iter()
        {
            if let Some(single_rules) = rules.get(item)
            {
                for rule in single_rules.iter()
                {
                    if single.contains(rule) && seen.contains(rule)
                    {
                        valid = false;
                        break;
                    }
                }
            }
            if !valid
            {
                break;
            }
            seen.insert(item);
        }
        if valid
        {
            sum += single.iter().nth(single.len() / 2).unwrap();
        }
    }
    sum
}

fn solve_second_part() -> usize
{
    let (update, rules) = get_formatted_data();

    let mut sum = 0;
    for single in update.iter()
    {
        let mut seen = HashSet::new();
        let mut valid = true;

        for item in single.iter()
        {
            if let Some(single_rules) = rules.get(item)
            {
                for rule in single_rules.iter()
                {
                    if single.contains(rule) && seen.contains(rule)
                    {
                        valid = false;
                        break;
                    }
                }
            }
            if !valid
            {
                break;
            }
            seen.insert(item);
        }
        if valid
        {
            continue;
        }

        let mut ptr = 0;
        let mut new_single = single.iter().rev().collect_vec();

        while ptr < new_single.len() - 1
        {
            let mut stop = false;

            while !stop
            {
                let r = rules.get(&new_single[ptr]);

                if r.is_none()
                {
                    break;
                }

                stop = true;

                for rule in r.unwrap().iter()
                {
                    if let Some(a) = new_single.iter().position(|r| *r == rule)
                    {
                        if a <= ptr
                        {
                            continue;
                        }
                        let tmp = new_single[ptr];
                        new_single[ptr] = new_single[a];
                        new_single[a] = tmp;
                        stop = false;
                        break;
                    }
                }
            }
            ptr += 1;
        }
        sum += new_single[new_single.len() / 2];
    }
    sum
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_part_solution()
    {
        let result = solve_first_part();

        assert_eq!(result, 5955);
    }

    #[test]
    fn test_second_part_solution()
    {
        let result = solve_second_part();

        assert_eq!(result, 4030);
    }
}
