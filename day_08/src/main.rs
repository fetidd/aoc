use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

static INPUT: &'static str = include_str!("../day_08.txt");

fn main() {
    let (instructions, map) = parse_input(&INPUT);
    println!("{}", find2(&instructions, &map));
}

fn find2(instructions: &[char], map: &HashMap<&str, (String, String)>) -> usize {
    let addrs: Vec<usize> = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|s| s.to_string())
        //     .collect_vec();
        // let ticks_to_finish = addrs
        //     .iter()
        .map(|x| ticks_to_finish(&instructions, &map, &x))
        .collect_vec();
    get_lcm(&addrs) as usize
}

fn get_lcm<T: num::Integer + Clone + Copy>(nums: &[T]) -> T {
    let mut nums: VecDeque<T> = VecDeque::from_iter(nums.into_iter().cloned());
    while nums.len() > 1 {
        let l = nums.pop_front().unwrap();
        let r = nums.pop_front().unwrap();
        let lcm = l.lcm(&r);
        nums.push_back(lcm);
    }
    nums[0]
}

fn ticks_to_finish<'a>(
    instructions: &[char],
    map: &'a HashMap<&'a str, (String, String)>,
    mut curr_addr: &'a str,
) -> usize {
    let mut i = 1;
    for instruction in instructions.into_iter().cycle() {
        let next = match instruction {
            'L' => &map.get(curr_addr).expect("missing addr").0,
            'R' => &map.get(curr_addr).expect("missing addr").1,
            _ => panic!("non L/R instruction!"),
        };
        if next.ends_with("Z") {
            break;
        }
        curr_addr = next;
        i += 1;
    }
    i
}

fn find(instructions: &[char], map: &HashMap<&str, (String, String)>) -> usize {
    let mut curr_addr = "AAA";
    let mut i = 1;
    for instruction in instructions.into_iter().cycle() {
        let next = match instruction {
            'L' => &map.get(curr_addr).expect("missing addr").0,
            'R' => &map.get(curr_addr).expect("missing addr").1,
            _ => panic!("non L/R instruction!"),
        };
        if next == "ZZZ" {
            break;
        }
        curr_addr = next;
        i += 1;
    }
    i
}

#[test]
fn test_find() {
    let input = "RL\n\
                 \n\
                 AAA = (BBB, CCC)\n\
                 BBB = (DDD, EEE)\n\
                 CCC = (ZZZ, GGG)\n\
                 DDD = (DDD, DDD)\n\
                 EEE = (EEE, EEE)\n\
                 GGG = (GGG, GGG)\n\
                 ZZZ = (ZZZ, ZZZ)";
    let (instructions, map) = parse_input(&input);
    dbg!(&instructions, &map);
    assert_eq!(2, find(&instructions, &map));
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<&str, (String, String)>) {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let map: HashMap<&str, (String, String)> = lines
        .skip(1)
        .map(|line| {
            let mut line = line.split(" = ");
            let addr = line.next().unwrap();
            let dirs = line
                .next()
                .unwrap()
                .split(", ")
                .map(|x| {
                    x.chars().filter(|c| c.is_ascii_uppercase()).fold(
                        String::new(),
                        |mut acc, el| {
                            acc.push(el);
                            acc
                        },
                    )
                })
                .collect_tuple::<(String, String)>()
                .unwrap();
            (addr, dirs)
        })
        .collect();
    (instructions, map)
}
