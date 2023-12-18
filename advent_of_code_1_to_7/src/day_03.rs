use std::ops::RangeInclusive;
use itertools::Itertools;

fn get_num_range(data: &[char], i: usize) -> RangeInclusive<usize> {
    let mut start = i;
    let mut end = i;
    for x_start in (0..=i).rev() {
        if !data[x_start].is_ascii_digit() {
            break;
        }
        start = x_start;
    }
    for x_end in i..data.len() {
        end = x_end;
        if !data[x_end].is_ascii_digit() {
            end -= 1;
            break;
        }
    }
    start..=end
}

fn char_slice_to_string(char_slice: &[char]) -> String {
    char_slice.iter().fold(String::new(), |mut s, ch| {
        s.push(*ch);
        s
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_b() {
        let data = std::fs::read_to_string("day_03.txt").unwrap();

        let line_length = data
            .chars()
            .position(|ch| ch == '\n')
            .expect("the schematic is one line!"); // assume all lines are same length, because they are :D
        let data_bytes = data.chars().filter(|ch| *ch != '\n').collect::<Vec<char>>(); //  remove newlines because we know the line length
        let mut main_index = 0;
        let mut ratio_sum = 0;
        while main_index < data_bytes.len() {
            let curr_item = data_bytes[main_index];
            let line_index = main_index
                .checked_rem(line_length)
                .expect("line length is zero somehow?!");
            let curr_line =
                &data_bytes[main_index - line_index..main_index - line_index + line_length];
            if curr_item == '*' {
                let mut ratios: Vec<i32> = vec![];
                let mut lines = vec![curr_line];
                if main_index >= line_length {
                    let line = &data_bytes[main_index - line_index - line_length..main_index - line_index];
                    lines.push(line);
                }
                if main_index < data_bytes.len() - line_length {
                    let line = &data_bytes[main_index - line_index + line_length..main_index - line_index + (line_length * 2)];
                    lines.push(line);
                }
                add_valid_gear_ratios(&mut ratios, &lines, line_index);
                if ratios.len() == 2 {
                    ratio_sum += ratios.iter().product::<i32>();
                }
                
            } 
            main_index += 1;
        }
        assert_eq!(73074886, ratio_sum);
    }

    fn add_valid_gear_ratios(ratios: &mut Vec<i32>, lines: &[&[char]], line_index: usize) {
        for line in lines.into_iter() {
            (*line)
                .iter()
                .enumerate()
                .filter(|(_, ch)| ch.is_ascii_digit())
                .map(|(index, _)| get_num_range(&line, index))
                .unique()
                .filter(|num_range| (line_index.saturating_sub(1)..=line_index+1).any(|x| num_range.contains(&x)))
                .for_each(|r| ratios.push(char_slice_to_string(&line[r]).parse::<i32>().expect("failed to parse found number")));
        }
    }

    #[test]
    fn solve_a() {
        let data = std::fs::read_to_string("day_03.txt").unwrap();
        let line_length = data
            .chars()
            .position(|ch| ch == '\n')
            .expect("the schematic is one line!"); // assume all lines are same length, because they are :D
        let data_bytes = data.chars().filter(|ch| *ch != '\n').collect::<Vec<char>>(); //  remove newlines because we know the line length
        let mut main_index = 0;
        let mut parts: Vec<i32> = vec![];
        while main_index < data_bytes.len() {
            let curr_item = data_bytes[main_index];
            let line_index = main_index
                .checked_rem(line_length)
                .expect("line length is zero somehow?!");
            let curr_line =
                &data_bytes[main_index - line_index..main_index - line_index + line_length];
            if curr_item.is_ascii_digit() {
                let num_range = get_num_range(&curr_line, line_index);
                let number = char_slice_to_string(&curr_line[num_range]);
                let mut checks = vec![];
                let check_left = |number_index: usize| line_index > 0 && number_index == 0;
                let check_right = |number_index: usize| {
                    line_index + number.len() < line_length && number_index == number.len() - 1
                };
                for number_index in 0..number.len() {
                    if check_left(number_index) {
                        checks.push(&curr_line[line_index - 1]);
                    }
                    if check_right(number_index) {
                        checks.push(&curr_line[line_index + number.len()]);
                    }
                    if main_index >= line_length {
                        if check_left(number_index) {
                            checks.push(&data_bytes[main_index + number_index - line_length - 1]);
                        }
                        checks.push(&data_bytes[main_index + number_index - line_length]);
                        if check_right(number_index) {
                            checks.push(&data_bytes[main_index + number_index - line_length + 1]);
                        }
                    }
                    if main_index < data_bytes.len() - line_length {
                        if check_left(number_index) {
                            checks.push(&data_bytes[main_index + number_index + line_length - 1]);
                        }
                        checks.push(&data_bytes[main_index + number_index + line_length]);
                        if check_right(number_index) {
                            checks.push(&data_bytes[main_index + number_index + line_length + 1]);
                        }
                    }
                }
                if checks
                    .into_iter()
                    .any(|c| c.is_ascii_punctuation() && *c != '.')
                {
                    parts.push(number.parse::<i32>().unwrap());
                }
                main_index += number.len();
            } else {
                main_index += 1;
            }
        }
        assert_eq!(527369, parts.iter().sum());
    }

    #[test]
    fn test_get_num_range() {
        let tests = vec![
            ("1", 0, (0..=0)),
            ("123", 0, (0..=2)),
            ("123", 1, (0..=2)),
            ("123", 2, (0..=2)),
            ("...123", 3, (3..=5)),
            ("...123", 4, (3..=5)),
            ("...123", 5, (3..=5)),
            ("...123..", 3, (3..=5)),
            ("...123..", 4, (3..=5)),
            ("...123..", 5, (3..=5)),
            ("...123\n", 3, (3..=5)),
            ("...123\n", 4, (3..=5)),
            ("...123\n", 5, (3..=5)),
            ("..\n123\n", 3, (3..=5)),
            ("..\n123\n", 4, (3..=5)),
            ("..\n123\n", 5, (3..=5)),
        ];
        for (test_num, (data, i, exp)) in tests.into_iter().enumerate() {
            let data_chars = data.chars().collect::<Vec<char>>();
            assert_eq!(
                exp,
                get_num_range(&data_chars, i),
                "test #{} failed",
                test_num + 1
            );
        }
    }
}
