use day_12::*;

static INPUT: &str = include_str!("../input.txt");
static TEST_INPUT: &str = include_str!("../test_input.txt");

fn main() {
    get_lines(TEST_INPUT)
        .into_iter()
        .for_each(|l| println!("{:?}", l));
}
