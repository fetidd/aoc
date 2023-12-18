use regex::{Captures, Regex};

#[cfg(test)]
mod solve {
    use super::*;

    #[test]
    fn solve_b() {
        let data = std::fs::read_to_string("day_04.txt").unwrap();
        let mut copies = [1; 193];
        for line in data.lines() {
            let caps = parse_card(line, CARD_PATTERN);
            let card_num = caps[1].parse::<usize>().expect("parsing card_num");
            let wins = check_wins(&caps, 10) as usize;
            let card_copies = copies[card_num - 1];
            dbg!(card_num, wins);
            for copy in copies[card_num..card_num + wins].iter_mut() {
                *copy += card_copies;
            }
            dbg!(copies);
        }
        assert_eq!(6283755, copies.iter().sum());
    }

    #[test]
    fn solve_a() {
        let data = std::fs::read_to_string("day_04.txt").unwrap();
        let mut total: i32 = 0;
        for line in data.lines() {
            let caps = parse_card(line, CARD_PATTERN);
            total += 2_f32.powi(check_wins(&caps, 10) - 1).floor() as i32;
        }
        assert_eq!(15268, total);
    }
}

static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
const CARD_PATTERN: &str = r"^Card\s+(\d+):\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+) \|\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)";
const CARD_PATTERN_TEST: &str = r"^Card\s+(\d+):\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+) \|\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s+(\d+)";
fn parse_card<'a>(card: &'a str, pattern: &'static str) -> Captures<'a> {
    let r = RE.get_or_init(|| Regex::new(pattern).expect("failed creating regex"));
    let caps = r.captures(card).expect("invalid line!");
    caps
}

fn check_wins(caps: &Captures, wins: usize) -> i32 {
    let winning: Vec<&str> = caps
        .iter()
        .skip(2)
        .take(wins)
        .map(|m| m.unwrap().as_str())
        .collect();
    caps.iter()
        .skip(wins + 2)
        .map(|m| m.unwrap().as_str())
        .filter(|x| winning.contains(x))
        .fold(0, |a, _| a + 1)
}
