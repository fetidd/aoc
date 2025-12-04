use std::ops::{Index, IndexMut};

pub fn run(input: &str, part: u8) -> String {
    let batt_len = if part == 1 { 2 } else { 12 };
    input.lines().map(|l| find_joltage(l, batt_len)).sum::<u64>().to_string()
}

const MAX_JOLTAGE: u8 = 57;
const ZERO: u8 = 48;

struct Batteries(Vec<(u8, usize)>);
impl Batteries {
    fn new(battery_num: usize) -> Self {
        Self(vec![(0, 0); battery_num])
    }

    fn zero(&mut self, index: usize) {
        for i in index..self.0.len() {
            self.0[i] = (0, 0);
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> impl Iterator<Item = &(u8, usize)> {
        self.0.iter()
    }

    fn into_iter(self) -> impl Iterator<Item = (u8, usize)> {
        self.0.into_iter()
    }
}

impl Index<usize> for Batteries {
    type Output = (u8, usize);

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Batteries {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

fn find_joltage(line: &str, battery_num: usize) -> u64 {
    let mut batteries = Batteries::new(battery_num);
    let mut first_battery_index = 0;
    let mut furthest_cursor_index = 0;
    let line = line.as_bytes();

    'line_loop: while first_battery_index < line.len() - 1 {
        let first_battery = line[first_battery_index];
        if first_battery > batteries[0].0 {
            batteries[0].0 = first_battery;
            // if first_battery_index >= furthest_cursor_index {
            //     batteries.zero(1);
            //     furthest_cursor_index = first_battery_index;
            // }
        }
        let mut cursor_index = first_battery_index + 1;
        while cursor_index < line.len() {
            let cursor_battery = line[cursor_index];
            if cursor_index < line.len() - (batteries.len() - 1) && cursor_battery > batteries[0].0 {// found a stronger battery to use as the first
                batteries[0].0 = cursor_battery;
                first_battery_index = cursor_index;
                batteries.zero(1);
                furthest_cursor_index = cursor_index;
                continue 'line_loop;
            }
            if cursor_battery > batteries[1].0{
                // new stronger second battery
                batteries[1].0 = cursor_battery;
                furthest_cursor_index = cursor_index;
            }
            if batteries[1].0 == MAX_JOLTAGE {
                break;
            }
            cursor_index += 1;
        }
        if batteries.iter().all(|(b, _)| *b == MAX_JOLTAGE) {
            break 'line_loop;
        }
        first_battery_index += 1;
    }
    batteries.into_iter().map(|(b, _)| (b - ZERO).to_string()).collect::<String>().parse().unwrap()
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
        // assert_eq!(987, find_joltage("987654321111111", 3));
        // assert_eq!(819, find_joltage("811111111111119", 3));
        // assert_eq!(478, find_joltage("234234234234278", 3));
        // assert_eq!(921, find_joltage("818181911112111", 3));
        // assert_eq!(922, find_joltage("121281212912121", 3));
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
