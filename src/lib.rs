use std::any::Any;

pub trait Year {
    fn year_number(&self) -> u16;

    fn days(&self) -> Vec<Box<dyn Day<T = dyn Any>>>;
}

pub trait Day {
    type T;

    fn day_number(&self) -> u8;

    fn adapt_input(&self, original_input: String) -> Self::T;
    fn part1(&self, input: &Self::T) -> String;
    fn part2(&self, input: &Self::T) -> String;
}
