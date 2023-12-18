use std::collections::VecDeque;

static INPUT: &'static str = include_str!("../day_09.txt");

fn main() {
    let answer: i64 = INPUT
        // let answer: i64 = "0 3 6 9 12 15\n\
        //                    1 3 6 10 15 21\n\
        //                    10 13 16 21 30 45"
        .lines()
        .map(|line| {
            let seq = line
                .split(" ")
                .map(|n| n.parse::<i64>().expect("parsing seq num"))
                .collect::<Vec<i64>>();
            dbg!(&line);
            let mut prev_steps = VecDeque::new();
            let mut steps = get_steps(&seq);
            loop {
                dbg!(&steps);
                let new_steps = get_steps(&steps);
                prev_steps.push_front(steps.clone());
                if new_steps.iter().all(|x| *x == 0) {
                    break;
                }
                steps = new_steps;
            }
            let mut curr_step = steps[0];
            dbg!(&curr_step);
            for st in prev_steps.iter().skip(1) {
                curr_step = st.first().unwrap() - curr_step;
                dbg!(&curr_step);
            }
            seq.first().unwrap() - curr_step
        })
        .sum();
    println!("answer = {}", answer);
}

fn get_steps(seq: &[i64]) -> Vec<i64> {
    let mut steps = vec![];
    let mut i = 0;
    while i < seq.len() - 1 {
        steps.push(seq[i + 1] - seq[i]);
        i += 1;
    }
    steps
}
