use crate::grid::Grid;

const PAPER_ROLL: char = '@';

pub fn run(input: &str, part: u8) -> String {
    let grid: Grid<char> = input.into();
    grid.iter().enumerate().map(|(i, p)| {
        if *p == PAPER_ROLL {
            let point = grid.get_point_from_index(i);
            let paper_neighbours: u32 = point
                .get_moore_neighbours(grid.width(), grid.height())
                .into_iter()
                .filter_map(|point| {
                    if point.row < grid.height() && point.col < grid.width() {
                        if grid[point] == PAPER_ROLL {
                            Some(1)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .sum();
            if paper_neighbours < 4 {
                return 1;
            }
        }
        0
    }).sum::<u32>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        println!("{input}");
        assert_eq!("13", &run(input, 1));
    }
}
