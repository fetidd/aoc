use std::{collections::HashSet, ops::RangeInclusive};

const DIVIDER: &str = "\n\n";
type R = RangeInclusive<u64>;

pub fn run(input: &str, part: u8) -> String {
    let div_i = input.trim().find(DIVIDER).unwrap();
    let (ranges, ids) = input.split_at(div_i);
    match part {
        1 => find_fresh(ranges, ids).len().to_string(),
        2 => find_all_possible_fresh(ranges)
            .into_iter()
            .flatten()
            .count()
            .to_string(),
        _ => todo!(),
    }
}

fn find_all_possible_fresh(ranges: &str) -> Vec<R> {
    let mut ranges = parse_ranges(ranges);
    let mut ranges_2 = vec![];
    loop {
        let pre = ranges.len();
        while let Some(mut r) = ranges.pop() {
            let mut i = ranges.len();
            while i > 0 {
                i -= 1;
                let curr = &ranges[i];
                if envelops(&r, curr) {
                    ranges.remove(i);
                } else if envelops(curr, &r) {
                    r = ranges.remove(i);
                } else if overlaps(&r, curr) || overlaps(curr, &r) {
                    let combined = *std::cmp::min(r.start(), curr.start())
                        ..=*std::cmp::max(r.end(), curr.end());
                    r = combined;
                    ranges.remove(i);
                }
            }
            ranges_2.insert(0, r);
        }
        if ranges_2.len() < pre {
            ranges = std::mem::take(&mut ranges_2); // move the merged ranges back over to the queue as we still made merges
        } else {
            // nothing changed so we're done
            break;
        }
    }
    if !ranges.is_empty() { ranges } else { ranges_2 }
}

fn envelops(l: &R, r: &R) -> bool {
    l.contains(r.start()) && l.contains(r.end())
}

fn overlaps(l: &R, r: &R) -> bool {
    l.contains(r.start()) ^ l.contains(r.end())
}

fn parse_ranges(ranges: &str) -> Vec<R> {
    ranges
        .lines()
        .map(|l| {
            let ends: Vec<_> = l.splitn(2, "-").collect();
            ends[0].parse::<u64>().unwrap()..=ends[1].parse::<u64>().unwrap()
        })
        .collect::<Vec<_>>()
}

fn find_fresh<'a>(ranges: &str, ids: &'a str) -> Vec<&'a str> {
    let ranges = parse_ranges(ranges);

    let is_fresh = |id: u64| {
        for range in ranges {
            if range.contains(&id) {
                return true;
            }
        }
        false
    };
    let mut fresh = HashSet::new();
    for id in ids.trim().lines() {
        let is_fresh = is_fresh.clone();
        if is_fresh(id.parse::<u64>().unwrap()) {
            fresh.insert(id);
        }
    }
    Vec::from_iter(fresh.into_iter())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_all_possible_fresh() {
        vec![
            ("1-10\n2-9\n3-8\n4-7\n5-6\n", vec![1..=10]),
            ("1-10\n2-9\n3-8\n11-14\n4-7\n5-6\n", vec![1..=10, 11..=14]),
            ("1-10\n2-9\n3-8\n10-14\n4-7\n5-6\n", vec![1..=14]),
            ("3-5\n10-14\n16-20\n12-18\n", vec![3..=5, 10..=20]),
            ("1-3\n2-6\n6-9\n12-15\n9-12\n", vec![1..=15]),
            ("1-3\n6-9\n2-6\n12-15\n9-12\n", vec![1..=15]),
            ("", vec![]),
            ("", vec![]),
        ]
        .into_iter()
        .for_each(|(ranges, exp)| assert_eq!(exp, find_all_possible_fresh(ranges)));
    }

    #[test]
    fn test_run() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("3", &run(input, 1));
        assert_eq!("14", &run(input, 2));
    }
}
