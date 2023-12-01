#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u32> {
    let find_first_digit = |c: &char| c.is_digit(10);

    input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_digit(10)))
        .map(|mut l| {
            let mut lr = l.clone().rev();
            
            u32::from_str_radix(format!("{}{}",
                l.find(find_first_digit).unwrap(),
                lr.find(find_first_digit).unwrap()
            ).as_str(), 10).unwrap()
        }
        )
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}
