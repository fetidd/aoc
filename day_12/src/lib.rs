mod item;
use item::Item;
use itertools::Itertools;

/// Function to get the day 12 lines in a tuple of items and groups from a string
pub fn get_lines(source: &str) -> Vec<(Vec<Item>, Vec<i32>)> {
    source
        .lines()
        .map(|line| {
            let (springs, groups) = line
                .split(" ")
                .collect_tuple::<(&str, &str)>()
                .expect("failed to create line tuple"); // panic if input badly formed
            (
                springs.chars().map(Item::from).collect::<Vec<Item>>(),
                groups
                    .split(",")
                    .map(|n| n.parse::<i32>().expect("failed parsing group"))
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}
