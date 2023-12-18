#[derive(Copy, Clone)]
struct Race {
    time_limit: u64,
    record: u64,
}
static INPUT_A: [Race; 4] = [
    Race {
        time_limit: 63,
        record: 411,
    },
    Race {
        time_limit: 78,
        record: 1274,
    },
    Race {
        time_limit: 94,
        record: 2047,
    },
    Race {
        time_limit: 68,
        record: 1035,
    },
];

static INPUT_B: [Race; 1] = [Race {
    time_limit: 63789468,
    record: 411127420471035,
}];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let error_margin = INPUT_B
            .into_iter()
            .map(solve_race)
            .fold(1, |acc, el| acc * el.len());
        assert_eq!(1, error_margin);
    }
}

fn solve_race(race: Race) -> Vec<u64> {
    let mut times = vec![];
    for hold_time in 0..=race.time_limit {
        let sail_time = race.time_limit - hold_time;
        let distance = hold_time * sail_time;
        if distance > race.record {
            times.push(hold_time);
        }
    }
    times
}

#[cfg(test)]
#[test]
fn test_solve_race() {
    // Time:      7  15   30
    // Distance:  9  40  200

    let race = Race {
        time_limit: 7,
        record: 9,
    };
    assert_eq!(Vec::from_iter(2..=5), solve_race(race));

    let race = Race {
        time_limit: 15,
        record: 40,
    };
    assert_eq!(Vec::from_iter(4..=11), solve_race(race));

    let race = Race {
        time_limit: 30,
        record: 200,
    };
    assert_eq!(Vec::from_iter(11..=19), solve_race(race));
}
