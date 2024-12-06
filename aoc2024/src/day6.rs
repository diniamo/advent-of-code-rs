use std::collections::HashSet;

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Obstacle,
    Guard,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start_row: isize,
    start_col: isize,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Map {
    let mut map = Map {
        tiles: Vec::new(),
        start_row: 0isize,
        start_col: 0isize,
    };

    for (r, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (c, chr) in line.chars().enumerate() {
            row.push(match chr {
                '.' => Tile::Empty,
                '#' => Tile::Obstacle,
                '^' => {
                    map.start_row = r as isize;
                    map.start_col = c as isize;

                    Tile::Guard
                },
                _ => unreachable!()
            })
        }

        map.tiles.push(row);
    }

    map
}

#[aoc(day6, part1)]
fn part1(input: &Map) -> usize {
    let mut positions: HashSet<(isize, isize)> = HashSet::new();

    let mut row = input.start_row;
    let mut col = input.start_col;
    let mut row_step = -1isize;
    let mut col_step = 0isize;

    loop {
        positions.insert((row, col));

        let mut row_new_i = row + row_step;
        let mut col_new_i = col + col_step;

        if row_new_i < 0 || col_new_i < 0 { break; }

        let row_new_u = row_new_i as usize;
        let col_new_u = col_new_i as usize;

        if row_new_u >= input.tiles.len() || col_new_u >= input.tiles[row_new_u].len() { break; }

        if input.tiles[row_new_u][col_new_u] == Tile::Obstacle {
            if col_step == 0 {
                col_step = -row_step;
                row_step = 0;
            } else {
                row_step = col_step;
                col_step = 0;
            }

            row_new_i = row + row_step;
            col_new_i = col + col_step;
        }

        row = row_new_i;
        col = col_new_i;
    }

    positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 41);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(&parse("<EXAMPLE>")), "<RESULT>");
    // }
}
