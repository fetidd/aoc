pub fn run(input: &str) -> String {
    input.lines().map(find_joltage).sum::<u64>().to_string()
}

const MAX_JOLTAGE: u8 = 57;
const ZERO: u8 = 48;

fn find_joltage(line: &str) -> u64 {
    let mut li = 0;
    let mut max_l = 0;
    let (mut max_r, mut max_ri) = (0, 0);
    let line = line.as_bytes();
    'outer: while li < line.len() - 1 {
        let l = line[li];
        if l > max_l {
            // new stronger left battery
            max_l = l;
            if li >= max_ri {
                // found a stronger left battery after the strongest right, so need a new right
                max_r = 0;
                max_ri = 0;
            }
        }
        let mut ri = li + 1;
        while ri < line.len() {
            let r = line[ri];
            if ri < line.len() - 1 && r > max_l {
                max_l = r;
                li = ri;
                max_r = 0;
                max_ri = 0;
                continue 'outer;
            }
            if r > max_r {
                // new stronger right battery
                max_r = r;
                max_ri = ri;
            }
            // println!("{max_l} at {li}, {max_r} at {ri} - {line:?}");
            if max_r == MAX_JOLTAGE {
                // strongest right battery we'll get
                break;
            }
            ri += 1;
        }
        if max_l == MAX_JOLTAGE && max_r == MAX_JOLTAGE {
            break; // got a 99, so cannot be a better battery in this line
        }
        li += 1;
    }
    format!("{}{}", max_l - ZERO, max_r - ZERO).parse().unwrap()
}

fn find_over_joltage(line: &str) -> u64 {
    let mut b = [0; 12];
    let mut li = 0;

    let check = || {
        let slots = b.len();
        for i in 0..slots {}
    };

    let line = line.as_bytes();
    'outer: while li < line.len() - 1 {
        let l = line[li];
        if l > max_l {
            // new stronger left battery
            max_l = l;
            if li >= max_ri {
                // found a stronger left battery after the strongest right, so need a new right
                max_r = 0;
                max_ri = 0;
            }
        }
        let mut ri = li + 1;
        while ri < line.len() {
            let r = line[ri];
            if ri < line.len() - 1 && r > max_l {
                max_l = r;
                li = ri;
                max_r = 0;
                max_ri = 0;
                continue 'outer;
            }
            if r > max_r {
                // new stronger right battery
                max_r = r;
                max_ri = ri;
            }
            // println!("{max_l} at {li}, {max_r} at {ri} - {line:?}");
            if max_r == MAX_JOLTAGE {
                // strongest right battery we'll get
                break;
            }
            ri += 1;
        }
        if max_l == MAX_JOLTAGE && max_r == MAX_JOLTAGE {
            break; // got a 99, so cannot be a better battery in this line
        }
        li += 1;
    }
    format!("{}{}", max_l - ZERO, max_r - ZERO).parse().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_joltage() {
        assert_eq!(98, find_joltage("987654321111111"));
        assert_eq!(89, find_joltage("811111111111119"));
        assert_eq!(78, find_joltage("234234234234278"));
        assert_eq!(92, find_joltage("818181911112111"));
        assert_eq!(92, find_joltage("121281212912121"));
    }

    #[test]
    fn test_find_over_joltage() {
        assert_eq!(987654321111, find_over_joltage("987654321111111"));
        assert_eq!(811111111119, find_over_joltage("811111111111119"));
        assert_eq!(434234234278, find_over_joltage("234234234234278"));
        assert_eq!(888911112111, find_over_joltage("818181911112111"));
    }

    #[test]
    fn test_run() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!("357", &run(input));
    }
}
