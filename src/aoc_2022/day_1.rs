use advent_of_code::Day;


pub struct Day1 {}

impl Day for Day1 {
    type T = String;

    fn day_number(&self) -> u8 {
        1
    }

    fn adapt_input(&self, original_input: String) -> Self::T {
        original_input
    }

    fn part1(&self, input: &Self::T) -> String {
        String::from("foo")
    }

    fn part2(&self, input: &Self::T) -> String {
        String::from("bar")
    }
}
