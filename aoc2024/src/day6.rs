use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone)]
enum Tile {
    Empty,
    Obstacle,
    Guard,
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    row_start: isize,
    col_start: isize,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Map {
    let mut map = Map {
        tiles: Vec::new(),
        row_start: 0isize,
        col_start: 0isize,
    };

    for (r, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (c, chr) in line.chars().enumerate() {
            row.push(match chr {
                '.' => Tile::Empty,
                '#' => Tile::Obstacle,
                '^' => {
                    map.row_start = r as isize;
                    map.col_start = c as isize;

                    Tile::Guard
                }
                _ => unreachable!(),
            })
        }

        map.tiles.push(row);
    }

    map
}

fn walk<T: FnMut(isize, isize, isize, isize) -> bool>(map: &Map, mut callback: T) {
    let mut row = map.row_start;
    let mut col = map.col_start;

    let mut row_step = -1isize;
    let mut col_step = 0isize;

    loop {
        let mut row_new_i = row + row_step;
        let mut col_new_i = col + col_step;

        if row_new_i < 0 || col_new_i < 0 {
            callback(row, col, row_step, col_step);
            break;
        }

        let mut row_new_u = row_new_i as usize;
        let mut col_new_u = col_new_i as usize;

        if row_new_u >= map.tiles.len() || col_new_u >= map.tiles[row_new_u].len() {
            callback(row, col, row_step, col_step);
            break;
        }

        while map.tiles[row_new_u][col_new_u] == Tile::Obstacle {
            if col_step == 0 {
                col_step = -row_step;
                row_step = 0;
            } else {
                row_step = col_step;
                col_step = 0;
            }

            row_new_i = row + row_step;
            col_new_i = col + col_step;

            row_new_u = row_new_i as usize; 
            col_new_u = col_new_i as usize;
        }

        if !callback(row, col, row_step, col_step) {
            break;
        }

        row = row_new_i;
        col = col_new_i;
    }
}

#[aoc(day6, part1)]
fn part1(input: &Map) -> usize {
    let mut positions: HashSet<(isize, isize)> = HashSet::new();

    walk(input, |row, col, _, _| {
        positions.insert((row, col));
        true
    });

    positions.len()
}

#[aoc(day6, part2)]
fn part2(input: &Map) -> usize {
    let mut count = 0usize;

    let mut checked_positions = vec![(input.row_start, input.col_start)];

    walk(input, |row, col, _, _| {
        if checked_positions.contains(&(row, col)) {
            return true;
        }

        let mut map = input.clone();
        map.tiles[row as usize][col as usize] = Tile::Obstacle;

        let mut positions = Vec::new();
        let mut is_first = true;

        walk(&map, |r, c, r_step, c_step| {
            if !is_first && positions.contains(&(r, c, r_step, c_step)) {
                count += 1;
                false
            } else {
                positions.push((r, c, r_step, c_step));
                is_first = false;
                true
            }
        });

        checked_positions.push((row, col));
        true
    });

    count
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 6);
    }
}
