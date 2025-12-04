pub fn run(input: &str, part: u8) -> String {
    let batt_len = if part == 1 { 2 } else { 12 };
    input.lines().map(|l| find_joltage(l, batt_len)).sum::<u64>().to_string()
}

const MAX_JOLTAGE: u8 = 57;
const ZERO: u8 = 48;

fn find_joltage(line: &str, battery_num: usize) -> u64 {
    let mut b = vec![0; battery_num];
    let mut li = 0;
    let mut max_ri = 0;
    let line = line.as_bytes();
    'outer: while li < line.len() - 1 {
        let l = line[li];
        if l > b[0] {
            // new stronger first battery
            b[0] = l;
            if li >= max_ri {
                // found a stronger first battery after the strongest second, so need a new right
                b[1] = 0;
                max_ri = 0;
            }
        }
        let mut ri = li + 1;
        while ri < line.len() {
            let r = line[ri];
            if ri < line.len() - 1 && r > b[0] {
                b[0] = r;
                li = ri;
                b[1] = 0;
                max_ri = 0;
                continue 'outer;
            }
            if r > b[1]{
                // new stronger second battery
                b[1] = r;
                max_ri = ri;
            }
            // println!("{max_l} at {li}, {max_r} at {ri} - {line:?}");
            if b[1] == MAX_JOLTAGE {
                // strongest second battery we'll get
                break;
            }
            ri += 1;
        }
        if b[0] == MAX_JOLTAGE && b[1] == MAX_JOLTAGE {
            break; // got a 99, so cannot be a better battery in this line
        }
        li += 1;
    }
    b.into_iter().map(|b| (b - ZERO).to_string()).collect::<String>().parse().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_joltage() {
        assert_eq!(98, find_joltage("987654321111111", 2));
        assert_eq!(89, find_joltage("811111111111119", 2));
        assert_eq!(78, find_joltage("234234234234278", 2));
        assert_eq!(92, find_joltage("818181911112111", 2));
        assert_eq!(92, find_joltage("121281212912121", 2));
        // assert_eq!(987654321111, find_joltage("987654321111111", 12));
        // assert_eq!(811111111119, find_joltage("811111111111119", 12));
        // assert_eq!(434234234278, find_joltage("234234234234278", 12));
        // assert_eq!(888911112111, find_joltage("818181911112111", 12));
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
