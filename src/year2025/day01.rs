pub fn run(input: &str) -> String {
    let mut ptr = 50;
    let mut zeroes = 0;
    for line in input.lines() {
        let initial_ptr = ptr;
        let (dir, ticks) = line.split_at(1);
        let ticks = ticks.parse::<i32>().unwrap();
        let op = match dir {
            "L" => i32::strict_sub,
            "R" => i32::strict_add,
            _ => panic!("invalid direction: {dir}"),
        };
        ptr = op(ptr, ticks % 100);
        let roll_overs = ticks.strict_div(100);
        zeroes += roll_overs;
        if ptr < 0 {
            ptr += 100;
            if initial_ptr != 0 {
                zeroes += 1;
             }
        } else if ptr >= 100 {
            ptr -= 100;
            if initial_ptr != 0 {
                zeroes += 1;
             }
        } else if ptr == 0 {
            zeroes += 1;
        }
    }
    format!("{zeroes}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "L68
L30
R48
L5
R600
L550
L1
L99
R14
L82";
        assert_eq!(String::from("6"), run(input));
    }
}
