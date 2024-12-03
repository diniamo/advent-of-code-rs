#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let mut index: usize = 0;
    let mut lhs = String::new();
    let mut rhs = String::new();

    let mut sum = 0u32;

    for c in input.chars() {
        if (index == 0 && c == 'm')
            || (index == 1 && c == 'u')
            || (index == 2 && c == 'l')
            || (index == 3 && c == '(')
        {
            index += 1;
        } else if index == 4 && c.is_ascii_digit() {
            lhs.push(c);
        } else if index == 4 && c == ',' {
            index += 1;
        } else if index == 5 && c.is_ascii_digit() {
            rhs.push(c);
        } else if index == 5 && c == ')' && !lhs.is_empty() && !rhs.is_empty() {
            sum += lhs.parse::<u32>().unwrap() * rhs.parse::<u32>().unwrap();

            index = 0;
            lhs.clear();
            rhs.clear();
        } else {
            index = 0;
            lhs.clear();
            rhs.clear();
        }

        println!(
            "{c} ({}) -> {index}, {lhs}, {rhs}, {sum}",
            c.is_ascii_digit()
        );
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 161);
    }
}
