use advent_of_code::Year;
use crate::aoc_2022::*;

mod aoc_2022;

fn main() {
    let years = [
        Aoc2022 {} 
    ];

    for year in years {
        println!("Running year {}:", year.year_number());

        let days = year.days();
        let input = String::from("temporairy input till i do it properly");

        for day in days.iter() {
            println!("  Day {}\n    Part 1: {}\n    Part 2: {}", day.day_number(), day.part1(&input), day.part2(&input));
        }
    }
}
