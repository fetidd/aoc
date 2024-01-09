mod spring;
use itertools::Itertools;
use spring::SpringGroup;

/// Function to get the day 12 lines in a tuple of items and groups from a string
pub fn get_lines(source: &str) -> Vec<(Vec<SpringGroup>, Vec<i32>)> {
    source
        .lines()
        .map(|line| {
            let (springs, groups) = line
                .split(" ")
                .collect_tuple::<(&str, &str)>()
                .expect("failed to create line tuple"); // panic if input badly formed
            (parse_springs(springs), parse_groups(groups))
        })
        .collect()
}

fn parse_groups(groups: &str) -> Vec<i32> {
    groups
        .split(",")
        .map(|n| n.parse::<i32>().expect("failed parsing group"))
        .collect::<Vec<_>>()
}

fn parse_springs(springs: &str) -> Vec<SpringGroup> {
    let mut i = 0usize;
    let mut groups = vec![];
    let springs = springs.chars().collect::<Vec<_>>();
    while i < springs.len() {
        // input will always be ascii so this is safe
        let g: SpringGroup = springs[i..].into();
        i += g.len();
        groups.push(g);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_springs() {}
}
