use std::collections::HashSet;

use advent_of_code::Day;


pub struct Day6 {}

fn has_dup(s: &str) -> bool {
    let set: HashSet<char> = HashSet::from_iter(s.chars());
    return s.len() != set.len();
}

impl Day for Day6 {
    fn day_number(&self) -> u8 {
        6
    }

    fn part1(&self, input: &String) -> String {
        for (i, c) in input.chars().enumerate() {
            if i >= 4 {
                if !has_dup(&input[(i - 4)..i]) {
                    return i.to_string()
                }
            }
        }
        unreachable!();
    }

    fn part2(&self, input: &String) -> String {
        for (i, c) in input.chars().enumerate() {
            if i >= 14 {
                if !has_dup(&input[(i - 14)..i]) {
                    return i.to_string()
                }
            }
        }
        unreachable!()
    }
}
