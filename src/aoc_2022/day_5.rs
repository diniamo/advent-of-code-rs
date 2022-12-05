use advent_of_code::Day;
use queues::{Queue, IsQueue};


pub struct Day5 {}

impl Day for Day5 {
    fn day_number(&self) -> u8 {
        5
    }

    fn part1(&self, input: &String) -> String {
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let mut table: Vec<Queue<char>> = vec![];
        let mut moves: Vec<(u8, u8, u8)> = Vec::new();
        let mut is_past = false;
        for line in input.lines() {
            if !line.trim().starts_with('[') {
                is_past = true;
                continue;
            }

            if is_past {
                let mut split = line.split(' ');
                moves.push((
                    split.nth(1).unwrap().parse().unwrap(),
                    split.nth(1).unwrap().parse().unwrap(),
                    split.nth(1).unwrap().parse().unwrap()
                ));
            } else {
                for (i, c) in line.chars().enumerate() {
                    if i == 0 { continue; }

                    let index = (i - 1) / 4;
                    if table.len() == index {
                        table.push(Queue::new());
                    }

                    if (i - 1) % 4 == 0 && c != ' ' {
                        _ = table[index].add(c);
                    }
                }
            }
        }

        for m in moves {
            for _ in 0..m.0 {
                table[usize::from(m.2 - 1)].add((&mut table[usize::from(m.1 - 1)]).remove().unwrap());
            }
        }

        String::new()
    }

    fn part2(&self, input: &String) -> String {
        String::new()
    }
}
