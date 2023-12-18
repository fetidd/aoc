const TOKENS: [&str; 19] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

#[derive(Debug)]
struct LineParse<'a> {
    left: (&'a str, Option<usize>),
    right: (&'a str, Option<usize>),
}

fn token_to_digit(t: &str) -> &str {
    dbg!(t);
    match t {
        "0" => "0",
        "one" | "1" => "1",
        "two" | "2" => "2",
        "three" | "3" => "3",
        "four" | "4" => "4",
        "five" | "5" => "5",
        "six" | "6" => "6",
        "seven" | "7" => "7",
        "eight" | "8" => "8",
        "nine" | "9" => "9",
        _ => unreachable!(),
    }
}

pub fn solve_01(input: &str) -> i32 {
    let mut answer = 0;
    for line in input.lines() {
        let found = TOKENS
            .into_iter()
            .map(|t| LineParse {
                left: (token_to_digit(t), line.find(t)),
                right: (token_to_digit(t), line.rfind(t)),
            })
            .filter(|LineParse { left, right: _ }| left.1.is_some())
            .collect::<Vec<LineParse>>();
        let new_val = format!(
            "{}{}",
            found
                .iter()
                .min_by(|x, y| x.left.1.partial_cmp(&y.left.1).unwrap())
                .unwrap()
                .left
                .0,
            found
                .iter()
                .max_by(|x, y| x.right.1.partial_cmp(&y.right.1).unwrap())
                .unwrap()
                .right
                .0
        )
        .parse::<i32>()
        .unwrap();
        answer += new_val;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_01() {
        let data = std::fs::read_to_string("day_01.txt").unwrap();
        assert_eq!(54087, solve_01(&data));
    }
}
