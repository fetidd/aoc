use crate::grid::Grid;

const PAPER_ROLL: char = '@';

pub fn run(input: &str, part: u8) -> String {
    let mut grid: Grid<char> = input.into();
    if part == 1 {
        grid.iter()
            .enumerate()
            .map(|(i, p)| {
                if *p == PAPER_ROLL && can_move_paper_roll(i, &grid) {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>()
            .to_string()
    } else {
        let mut removed = 0;
        let mut removed_this_pass = true;
        while removed_this_pass {
            removed_this_pass = false;
            // grid.iter_mut().enumerate().for_each(|(i, p)| {
            //     if *p == PAPER_ROLL && can_move_paper_roll(i, &grid) {
            //         *p = '.';
            //         removed += 1;
            //         removed_this_pass = true;
            //     }
            // })
            for i in 0..grid.len() {
                let p = grid.get_point_from_index(i);
                if grid[p] == PAPER_ROLL && can_move_paper_roll(i, &grid) {
                    grid[p] = '.';
                    removed += 1;
                    removed_this_pass = true;
                }
            }
        }
        removed.to_string()
    }
}

fn can_move_paper_roll(i: usize, grid: &Grid<char>) -> bool {
    grid.get_point_from_index(i)
        .get_moore_neighbours(grid.width(), grid.height())
        .into_iter()
        .map(|point| if grid[point] == PAPER_ROLL { 1 } else { 0 })
        .sum::<u32>()
        < 4
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
        assert_eq!("43", &run(input, 2));
    }
}
