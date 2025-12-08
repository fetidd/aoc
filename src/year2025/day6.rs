pub fn run(input: &str, part: u8) -> String {
    let parser = match part {
        1 => parse_maths_homework_1,
        2 => parse_maths_homework_2,
        _ => panic!(),
    };
    parser(input)
        .into_iter()
        .map(|(operands, operator)| {
            let first = operands[0];
            operands
                .into_iter()
                .skip(1)
                .fold(first, |acc, e| operator(acc, e))
        })
        .sum::<u64>()
        .to_string()
}

type Problem = (Vec<u64>, fn(u64, u64) -> u64);

fn parse_maths_homework_1(input: &str) -> Vec<Problem> {
    let mut operands = Vec::new();
    let mut operators = Vec::new();
    for line in input.lines() {
        for (i, s) in line.split_whitespace().enumerate() {
            if is_operator(s) {
                operators.push(get_operator_fn(s));
            } else {
                if operands.len() < i + 1 {
                    operands.push(Vec::new());
                }
                let parsed: u64 = s.parse().expect("parsing operand failed");
                operands[i].push(parsed);
            }
        }
    }
    let mut problems: Vec<Problem> = Vec::new();
    for (i, operand_list) in operands.into_iter().enumerate() {
        problems.push((operand_list, operators[i]));
    }
    problems
}

fn str_to_ch(s: &str) -> char {
    s.chars().nth(0).expect("empty op str")
}

fn get_operator_fn(op: &str) -> fn(u64, u64) -> u64 {
    get_operator_fn_ch(&str_to_ch(op))
}

fn get_operator_fn_ch(op: &char) -> fn(u64, u64) -> u64 {
    match op {
        '+' => u64::strict_add,
        '*' => u64::strict_mul,
        '-' => u64::strict_sub,
        '/' => u64::strict_div,
        _ => panic!("bad operator"),
    }
}

fn is_operator(s: &str) -> bool {
    is_operator_ch(&str_to_ch(s))
}

fn is_operator_ch(ch: &char) -> bool {
    ['+', '*', '-', '/'].contains(ch)
}

fn parse_maths_homework_2(input: &str) -> Vec<Problem> {
    let mut problems: Vec<Problem> = Vec::new();
    let mut lines = input.lines().rev();
    let operator_line = lines.next().expect("empty input");
    let mut last_i = 0;
    let mut last_gap = 0;
    // use last line to get operators and calculate column width
    for (i, ch) in operator_line.char_indices() {
        if is_operator_ch(&ch) {
            let gap = i - last_i;
            if last_gap > 0 && gap != last_gap {
                panic!("not spaced evenly");
            }
            last_gap = gap;
            last_i = i;
            problems.push((Vec::new(), get_operator_fn_ch(&ch)));
        }
    }
    // collect operands as strings of col length including whitespace
    let mut operands = vec![];
    let col_length = last_gap;
    for mut line in lines {
        let mut cols = Vec::new();
        while !line.is_empty() {
            let is_last = line.len() <= col_length;
            let mut split_i = col_length;
            if is_last {
                split_i = col_length - 1;
            }
            let (col, rest) = line.split_at(split_i);
            println!("{line} -> {col}, {rest}");
            let mut to_add = col;
            if !is_last {
                to_add = &to_add[0..to_add.len() - 1];
            }
            cols.push(to_add);
            line = rest;
        }
        operands.push(cols);
    }
    for (prob_i, problem) in problems.iter_mut.enumerate() {
        for col_i in 0..col_length {
            forn
        }
    }
    dbg!(operands);
    problems
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_maths_homework_1() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let exp: Vec<Problem> = vec![
            (vec![123, 45, 6], u64::strict_mul),
            (vec![328, 64, 98], u64::strict_add),
            (vec![51, 387, 215], u64::strict_mul),
            (vec![64, 23, 314], u64::strict_add),
        ];
        assert_eq!(exp, parse_maths_homework_1(input));
    }

    #[test]
    fn test_parse_maths_homework_2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let exp: Vec<Problem> = vec![
            (vec![356, 24, 1], u64::strict_add),
            (vec![8, 248, 369], u64::strict_mul),
            (vec![175, 581, 32], u64::strict_add),
            (vec![4, 431, 623], u64::strict_mul),
        ];
        assert_eq!(exp, parse_maths_homework_2(input));
    }

    #[test]
    fn test_run() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!("4277556", &run(input, 1));
        assert_eq!("3263827", &run(input, 2));
    }
}
