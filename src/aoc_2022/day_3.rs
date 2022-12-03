use advent_of_code::Day;


pub struct Day3 {}

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
// s1 and s2 are always the same length
fn intersection(strs: Vec<&str>) -> char {
    for c in strs.first().unwrap().chars() {
        if strs[1..].iter().all(|line| line.contains(c)) {
            return c
        }
    }

    panic!("Not possible");
}

impl Day for Day3 {
    fn day_number(&self) -> u8 {
        3
    }


    fn part1(&self, input: &String) -> String {
        let bags = input.trim().split('\n').map(|line| (&line[..(line.len()/2)], &line[(line.len()/2)..])).collect::<Vec<(&str, &str)>>();

        let mut prio_sum = 0;
        for bag in bags {
            prio_sum += ALPHABET.find(intersection(vec![bag.0, bag.1])).unwrap() + 1;
        }

        prio_sum.to_string()
    }

    fn part2(&self, input: &String) -> String {
        let lines = input.trim().split('\n').collect::<Vec<&str>>();

        let mut groups = Vec::new();

        for i in 0..lines.len() {
            if (i + 1) % 3 == 0 {
                groups.push((lines[i - 2], lines[i - 1], lines[i]));
            }
        }

        let mut prio_sum = 0;
        for group in groups {
            prio_sum += ALPHABET.find(intersection(vec![group.0, group.1, group.2])).unwrap() + 1;
        }

        prio_sum.to_string()
    }
}
