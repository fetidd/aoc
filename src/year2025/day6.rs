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
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let line_len = lines[0].len();
    let num_lines = lines.len();
    let mut i = 0;
    let mut operator = None;
    let mut operands = vec![];
    while i < line_len {
        let num_cursors = num_lines - 1;
        let mut grabbed = vec![' '; num_cursors];
        for (line_i, cursor) in grabbed.iter_mut().enumerate() {
            // let index = i + (line_len * i);
            let ch = lines[line_i][i];
            *cursor = ch;
        }
        let grabbed = grabbed.iter().collect::<String>();
        if !grabbed.trim().is_empty() {
            operands.push(grabbed.trim().parse::<u64>().unwrap())
        } else {
            push_problem(&mut operands, &mut operator, &mut problems);
        }
        let ch = lines[num_lines - 1][i];
        if is_operator_ch(&ch) {
            operator = Some(get_operator_fn_ch(&ch));
        }

        i += 1;
    }
    push_problem(&mut operands, &mut operator, &mut problems);
    problems
}

fn push_problem(
    operands: &mut Vec<u64>,
    operator: &mut Option<fn(u64, u64) -> u64>,
    problems: &mut Vec<Problem>,
) {
    let problem = (std::mem::take(operands), operator.take().unwrap());
    problems.push(problem);
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
            (vec![1, 24, 356], u64::strict_mul),
            (vec![369, 248, 8], u64::strict_add),
            (vec![32, 581, 175], u64::strict_mul),
            (vec![623, 431, 4], u64::strict_add),
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
