use advent_of_code::Day;


pub struct Day8 {}

impl Day for Day8 {
    fn day_number(&self) -> u8 {
        8
    }

    fn part1(&self, input: &String) -> String {
//         let input = "30373
// 25512
// 65332
// 33549
// 35390";
        let map: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

        map.iter().map(|row| row.iter().map(|digit| digit.to_string()).collect()).map(|row: Vec<String>| row.join(" ")).for_each(|line| println!("{line}"));
        println!();


        // The initial value here is the perimeter of the grid, as we don't account for that in the loop
        let mut visible: usize = 2*(map.len() - 2 + map[0].len() - 2) + 4;
        println!("{visible}");
        for (i, row) in (&map[1..(map.len() - 1)]).iter().enumerate() {
            for (j, &digit) in (&row[1..(row.len() - 1)]).iter().enumerate() {
                print!("{digit} at ({j}, {i}) in row {:?}", row);
                print!(" with if values {}, {}, {}, {}", (&row[..=j]).iter().all(|&d| d < digit), (&row[(j + 2)..]).iter().all(|&d| d < digit), (&map[..=i]).iter().all(|r| r[j + 1] < digit), (&map[(i + 2)..]).iter().all(|r| r[j + 1] < digit));

                // Left, right, up, down
                if (&row[..=j]).iter().all(|&d| d < digit)    // Wouldn't be inclusive, but we start from row[1] and j is still 0
                    || (&row[(j + 2)..]).iter().all(|&d| d < digit)    // Would be j+1, reason above
                    || (&map[..=i]).iter().all(|r| r[j + 1] < digit)    // Wouldn't be inclusive, but we start from map[1] and i is still 0
                    || (&map[(i + 2)..]).iter().all(|r| r[j + 1] < digit) {    // Would be i+1 and j, reason(s) above
                    visible += 1;
                    println!(" is visible!")
                } else { println!(); } 
                
            }
        }
        
        visible.to_string()
    }

    fn part2(&self, input: &String) -> String {
        String::new()
    }
}
