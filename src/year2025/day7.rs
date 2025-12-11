use std::collections::HashSet;

use crate::{grid::Grid, point::Point};

pub fn run(input: &str, part: u8) -> String {
    match part {
        1 => part_1(input).to_string(),
        _ => panic!(),
    }
}

fn part_1(input: &str) -> u32 {
    let grid = Grid::from(input);
    let start = grid.get_point_from_index(input.find('S').unwrap());
    let mut line_i = 1;
    let mut split_c = 0;
    let mut beams = HashSet::new();
    beams.insert(start);
    while line_i < grid.height() {
        for i in 0..grid.width() {
            let check_p = (line_i, i);
            if grid[check_p] == '^' {
                for beam in beams.clone().iter() {
                    if beam.col == check_p.1 {
                        split_c += 1;
                        beams.remove(&beam);
                        beams.insert(Point::new(beam.row, beam.col - 1));
                        beams.insert(Point::new(beam.row, beam.col + 1));
                    }
                }
            }
        }
        line_i += 1;
    }
    split_c
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!("21", &run(input, 1));
        assert_eq!("40", &run(input, 2));
    }
}
