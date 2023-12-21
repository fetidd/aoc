static INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_expand_universe() {

}

fn expand_universe(universe: &mut Universe) {
    
}

struct Universe(Vec<Vec<char>>);

impl Universe {
    fn from(input: &str) -> Self {
        Self(input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>())
    }
}
