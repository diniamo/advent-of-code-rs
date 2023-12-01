#[aoc_generator(day1)]
pub fn input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_digit(10)))
        .map(|mut l| u32::from_str_radix(format!("{}{}", l.next().unwrap(), l.last().unwrap()).as_str(), 10).unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}
