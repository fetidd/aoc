use std::ops::RangeInclusive;

pub fn run(input: &str) -> String {
    input.trim().split(",").map(|range_str| {
        let dash_index = range_str.find("-").expect("range string was missing a dash!");
        let (left, right) = range_str.split_at(dash_index);
        let right = &right[1..];
        (range_str, left.parse::<i64>().unwrap()..=right.parse::<i64>().unwrap())
    })
    .map(calculate_range)
    .map(|v| v.into_iter().sum::<i64>())
    .sum::<i64>()
    .to_string()
}

fn calculate_range((_range_str, range): (&str, RangeInclusive<i64>)) -> Vec<i64> {
    range.filter_map(|n| {
        let n_str = n.to_string();
        let n_len = n_str.len();
        let mods = 1..=(n_len / 2);
        for m in mods {
            if n_len % m == 0 {
                let mut invalid = true;
                let first_chunk = &n_str[..m];
                let mut check_range = m..(2*m);
                while check_range.end <= n_len {
                    let check_chunk = &n_str[check_range.clone()];
                    if first_chunk != check_chunk {
                        invalid = false;
                        break;
                    }
                    check_range = (check_range.end)..(check_range.end + m);
                }
                if invalid {
                    return Some(n_str);
                }
            }
        }
        None
    })
    .map(|n_str| n_str.parse::<i64>().unwrap())
    .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        // assert_eq!(run(input), "1227775554");
        assert_eq!(run(input), "4174379265");
    }

    #[test]
    fn test_calculate_range() {
        let range = ("11-22", 11..=22);
        assert_eq!(vec![11, 22], calculate_range(range));
        let range = ("95-115", 95..=115);
        assert_eq!(vec![99, 111], calculate_range(range));
    }
}
