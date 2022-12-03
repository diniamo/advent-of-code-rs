use advent_of_code::*;

mod day_1;
mod day_2;

pub struct Aoc2022 {}
impl Year for Aoc2022 {
    fn year_number(&self) -> u16 {
        2022
    }

    fn days(&self) -> Vec<Box<dyn Day>> {
        vec![
            Box::new(day_1::Day1 {}),
            Box::new(day_2::Day2 {})
        ]
    }
}
