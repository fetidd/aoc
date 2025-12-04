pub fn run(input: &str, part: u8) -> String {
    let batt_len = if part == 1 { 2 } else { 12 };
    input
        .lines()
        .map(|l| find_joltage(l, batt_len))
        .sum::<u64>()
        .to_string()
}

fn find_joltage(line: &str, battery_num: usize) -> u64 {
    let mut batteries = vec![0; battery_num];
    let line: Vec<_> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut ci = 0;
    while ci < line.len() {
        let c = line[ci];
        for bi in 0..battery_num {
            if line.len() - ci >= battery_num - bi && c > batteries[bi] {
                batteries[bi] = c;
                for i in bi + 1..battery_num {
                    batteries[i] = 0;
                }
                break;
            }
        }
        ci += 1;
    }
    batteries
        .into_iter()
        .map(|b| (b).to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_joltage() {
        assert_eq!(98, find_joltage("987654321111111", 2));
        assert_eq!(89, find_joltage("819", 2));
        assert_eq!(89, find_joltage("811111111111119", 2));
        assert_eq!(78, find_joltage("234234234234278", 2));
        assert_eq!(92, find_joltage("818181911112111", 2));
        assert_eq!(92, find_joltage("121281212912121", 2));
        assert_eq!(987, find_joltage("987654321111111", 3));
        assert_eq!(819, find_joltage("811111111111119", 3));
        assert_eq!(478, find_joltage("234234234234278", 3));
        assert_eq!(921, find_joltage("818181911112111", 3));
        assert_eq!(922, find_joltage("121281212912121", 3));
        assert_eq!(987654321111, find_joltage("987654321111111", 12));
        assert_eq!(811111111119, find_joltage("811111111111119", 12));
        assert_eq!(434234234278, find_joltage("234234234234278", 12));
        assert_eq!(888911112111, find_joltage("818181911112111", 12));
    }

    #[test]
    fn test_run() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("357", &run(input, 1));
    }
}
