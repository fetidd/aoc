use day_12::*;

static INPUT: &str = include_str!("../input.txt");

fn main() {
    get_lines(INPUT)
        .into_iter()
        .for_each(|l| println!("{:?}", l));
}