use crate::grid::Grid;

pub fn run(input: &str, _part: u8) -> String {
    let grid = Grid::from(input);
    for (ri, row) in grid.iter().enumerate() {
        for (pi, plant) in row.iter().enumerate() {
            println!("{ri},{pi} {plant}");
        }
    }
    input.into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", &run(input, 1));
        // assert_eq!("1206", &run(input));
    }
}
