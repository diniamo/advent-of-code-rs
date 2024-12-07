use itertools::Itertools;
use std::iter;

#[derive(Debug)]
enum Op {
    Addition,
    Multiplication,
    Concatenation
}

fn concat(a: u64, b: u64) -> u64 {
    let mut pow = 10u64;

    while b >= pow {
        pow *= 10;
    }

    a * pow + b
}

fn process(equations: &[(u64, Vec<u64>)], operations: &[Op]) -> u64 {
    equations.iter().filter_map(|equation| {
        let mut result = equation.1[0];

        for ops in iter::repeat_n(operations.iter(), equation.1.len() - 1).multi_cartesian_product() {
            for i in 1..equation.1.len() {
                let n = equation.1[i];
                match ops[i - 1] {
                    Op::Addition => result += n,
                    Op::Multiplication => result *= n,
                    Op::Concatenation => result = concat(result, n)
                }
            }

            if result == equation.0 {
                return Some(result)
            }

            result = equation.1[0];
        }

        None
    }).sum()
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.lines().map(|line| {
        let (result, numbers) = line.split_once(": ").unwrap();

        (
            result.parse::<u64>().unwrap(),
            numbers.split(' ').map(|n| n.parse::<u64>().unwrap()).collect()
        )
    }).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[(u64, Vec<u64>)]) -> u64 {
    process(input, &[Op::Addition, Op::Multiplication])
}

#[aoc(day7, part2)]
fn part2(input: &[(u64, Vec<u64>)]) -> u64 {
    process(input, &[Op::Addition, Op::Multiplication, Op::Concatenation])
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 3749);
    }
}
