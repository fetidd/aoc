use day_12::*;

static INPUT: &str = include_str!("../input.txt");
static TEST_INPUT: &str = "???.### 1,1,3\n\
                           .??..??...?##. 1,1,3\n\
                           ?#?#?#?#?#?#?#? 1,3,1,6\n\
                           ????.#...#... 4,1,1\n\
                           ????.######..#####. 1,6,5\n\
                           ?###???????? 3,2,1";

fn main() {
    get_lines(TEST_INPUT)
        .into_iter()
        .for_each(|l| println!("{:?}", l));
}