use std::{ops::Range, collections::{HashSet, VecDeque}};

use itertools::Itertools;

#[cfg(test)]
mod solve {
    use super::*;

    #[test]
    fn solve_a() {
        let data = std::fs::read_to_string("day_05.txt").unwrap();
        assert_eq!(157211394, get_closest_location_a(&data));
    }

    #[test]
    fn solve_b() {
        let data = std::fs::read_to_string("day_05.txt").unwrap();
        assert_eq!(1, get_closest_location_b(&data));
    }
}

fn get_closest_location_a(almanac: &str) -> i64 {
    let mut almanac = almanac.lines().collect::<Vec<_>>();
    let seeds = parse_seeds(almanac.remove(0));
    almanac.remove(0);
    let mappings = almanac
        .split(|x| x.is_empty())
        .map(|chunk| parse_almanac_chunk(chunk))
        .collect::<Vec<_>>()
        .into_iter()
        .map(mapping::Mapping::from)
        .collect::<Vec<_>>();
    seeds
        .into_iter()
        .map(|mut seed| {
            mappings.iter().for_each(|m| seed = m.apply(seed));
            seed
        })
        .inspect(|s| eprintln!("{:?}", s))
        .fold(i64::MAX, |acc, el| std::cmp::min(acc, el))
}

fn parse_seeds(seed_line: &str) -> Vec<i64> {
    seed_line
        .split(" ")
        .map(str::parse::<i64>)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
}

fn get_closest_location_b(almanac: &str) -> i64 {
    let mut almanac = almanac.lines().collect::<Vec<_>>();
    let seed_line = almanac.remove(0);
    almanac.remove(0);
    let mut seeds = parse_seed_ranges(seed_line);
    let mappings = almanac
        .split(|x| x.is_empty())
        .map(|chunk| parse_almanac_chunk(chunk))
        .collect::<Vec<_>>()
        .into_iter()
        .map(mapping::Mapping::from)
        .collect::<Vec<_>>();
    for level in mappings {
        eprintln!("new level");
        seeds = level.apply_range(seeds);
    }
    seeds
        .iter()
        .map(|x| x.start)
        .inspect(|s| eprintln!("{:?}", s))
        .filter(|x| *x > 0)
        .fold(i64::MAX, |acc, el| std::cmp::min(acc, el))
}

fn parse_seed_ranges(seed_line: &str) -> VecDeque<Range<i64>> {
    let seed_line: Vec<i64> = seed_line
        .split(" ")
        .map(str::parse::<i64>)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();
    seed_line
        .as_slice()
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect::<VecDeque<_>>()
}

fn overlaps(l: &Range<i64>, r: &Range<i64>) -> bool {
    l.contains(&r.start)
        || l.contains(&(r.end - 1))
        || r.contains(&l.start)
        || r.contains(&(l.end - 1))
}

fn offset_range(range: &Range<i64>, offset: i64) -> Range<i64> {
    (range.start + offset)..(range.end + offset)
}

fn split_range(range: &Range<i64>, at: i64) -> (Range<i64>, Range<i64>) {
    (range.start..at, at..range.end)
}

mod mapping {
    use std::{collections::{VecDeque, HashSet, HashMap}, ops::Range};

    use crate::day_05::offset_range;

    use super::split_range;

    #[derive(Debug, PartialEq)]
    pub struct Mapping {
        maps: Vec<SubMapping>,
    }

    impl Mapping {
        pub fn apply(&self, num: i64) -> i64 {
            let mut map_num = num;
            for mapping in &self.maps {
                if mapping.range.contains(&num) {
                    map_num = mapping.apply(num);
                }
            }
            map_num
        }

        pub fn apply_range(&self, mut to_process: VecDeque<Range<i64>>) -> VecDeque<Range<i64>> {
            let mut deduper = HashMap::<Range<i64>, bool>::new();
            let mut processed = HashSet::new();
            while !to_process.is_empty() {
                let range = to_process.pop_front().unwrap(); // wont be empty
                let mut found_for_range = false;
                let mut bypassed: Option<Range<i64>> = None;
                for mapping in &self.maps {
                    let res = mapping.apply_range(range.clone());
                    if let Some(bypassed_inner) = res.bypassed {
                        bypassed = Some(bypassed_inner);
                    }
                    if let Some(mapped_inner) = res.mapped {
                        processed.insert(mapped_inner);
                        found_for_range = true;
                        if res.unmapped_left.is_none() && res.unmapped_right.is_none() { // mapped everything we can
                            break;
                        }
                    }
                    if let Some(unmapped_inner) = res.unmapped_left {
                        if !deduper.contains_key(&unmapped_inner) {
                            deduper.insert(unmapped_inner.clone(), true);
                            to_process.push_back(unmapped_inner);
                        }
                    }
                    if let Some(unmapped_inner) = res.unmapped_right {
                        if !deduper.contains_key(&unmapped_inner) {
                            deduper.insert(unmapped_inner.clone(), true);
                            to_process.push_back(unmapped_inner);
                        }
                    }
                }
                if !found_for_range {
                    processed.insert(bypassed.unwrap());
                }
            }
            VecDeque::from_iter(processed)
        }

        pub fn overlap(&self, num_range: Range<i64>) -> bool {
            for mapping in &self.maps {
                if super::overlaps(&num_range, &mapping.range) {
                    return true;
                }
            }
            false
        }
    }

    impl From<&[SubMapping]> for Mapping {
        fn from(maps: &[SubMapping]) -> Self {
            Self {
                maps: maps.to_vec(),
            }
        }
    }

    impl From<Vec<SubMapping>> for Mapping {
        fn from(maps: Vec<SubMapping>) -> Self {
            Self { maps }
        }
    }
    #[derive(Debug, PartialEq, Clone)]
    pub struct SubMapping {
        range: Range<i64>,
        offset: i64,
        raw: (i64, i64, i64),
    }

    #[derive(Debug)]
    pub struct RangeMappingResults {
        unmapped_left: Option<Range<i64>>,
        mapped: Option<Range<i64>>,
        unmapped_right: Option<Range<i64>>,
        bypassed: Option<Range<i64>>
    }

    impl SubMapping {
        pub fn apply(&self, num: i64) -> i64 {
            if self.range.contains(&num) {
                num + self.offset
            } else {
                num
            }
        }

        pub fn apply_range(&self, range: Range<i64>) -> RangeMappingResults {
            if range == self.range {
                RangeMappingResults {
                    unmapped_left: None,
                    mapped: Some(offset_range(
                        &(self.range.start..self.range.end),
                        self.offset,
                    )),
                    unmapped_right: None,
                    bypassed: None
                }
            } else if range.contains(&self.range.start) && range.contains(&(self.range.end - 1)) {
                RangeMappingResults {
                    unmapped_left: Some(range.start..self.range.start),
                    mapped: Some(offset_range(
                        &(self.range.start..self.range.end),
                        self.offset,
                    )),
                    unmapped_right: Some(self.range.end..range.end),
                    bypassed: None
                }
            } else if self.range.contains(&range.start) && self.range.contains(&(range.end - 1)) {
                RangeMappingResults {
                    unmapped_left: None,
                    mapped: Some(offset_range(&range, self.offset)),
                    unmapped_right: None,
                    bypassed: None
                }
            } else if self.range.contains(&range.start) {
                let (affected, unaffected) = split_range(&range, self.range.end);
                RangeMappingResults {
                    unmapped_left: None,
                    mapped: Some(offset_range(&affected, self.offset)),
                    unmapped_right: Some(unaffected),
                    bypassed: None
                }
            } else if self.range.contains(&(range.end - 1)) {
                let (unaffected, affected) = split_range(&range, self.range.start);
                RangeMappingResults {
                    unmapped_left: Some(unaffected),
                    mapped: Some(offset_range(&affected, self.offset)),
                    unmapped_right: None,
                    bypassed: None
                }
            } else { // if no offsetting happened to any of the range, it's considered "bypassed"
                RangeMappingResults {
                    unmapped_left: None,
                    mapped: None,
                    unmapped_right: None,
                    bypassed: Some(range)
                }
            }
        }
    }

    impl From<(i64, i64, i64)> for SubMapping {
        fn from((destination, source, n): (i64, i64, i64)) -> Self {
            Self {
                range: source..source + n,
                offset: destination - source,
                raw: (destination, source, n),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        // #[test]
        // fn test_submapping_apply_range() {
        //     let s = SubMapping::from((52, 50, 48));
        //     assert_eq!(s.apply_range(0..50), vec![0..50]);
        //     assert_eq!(s.apply_range(98..100), vec![98..100]);
        //     assert_eq!(s.apply_range(60..70), vec![62..72]);
        //     assert_eq!(s.apply_range(40..60), vec![40..50, 52..62]);
        //     assert_eq!(s.apply_range(90..100), vec![92..100, 98..100]);
        // }

        #[test]
        fn test_mapping_apply() {
            let m = Mapping::from(
                vec![
                    SubMapping::from((50, 98, 2)),
                    SubMapping::from((52, 50, 48)),
                ]
                .as_slice(),
            );
            for i in 0..50 {
                assert_eq!(m.apply(i), i);
            }
            for i in 50..98 {
                assert_eq!(m.apply(i), i + 2);
            }
            assert_eq!(m.apply(98), 50);
            assert_eq!(m.apply(99), 51);
        }

        #[test]
        fn test_mapping_apply_range() {
            let m = Mapping::from(
                vec![
                    SubMapping::from((30, 10, 15)), // offset 10..25 by 20
                    SubMapping::from((20, 30, 35)), // offset 30..65 by -10
                ]
                .as_slice(),
            );
            assert_eq!(m.apply_range(VecDeque::from([0..10])), VecDeque::from([0..10]));
            assert_eq!(m.apply_range(VecDeque::from([0..15])), VecDeque::from([30..35, 0..10]));
            assert_eq!(m.apply_range(VecDeque::from([10..25])), VecDeque::from([30..45]));
            assert_eq!(m.apply_range(VecDeque::from([20..30])), VecDeque::from([40..45, 25..30]));
            assert_eq!(m.apply_range(VecDeque::from([30..50])), VecDeque::from([20..40]));
            assert_eq!(m.apply_range(VecDeque::from([40..70])), VecDeque::from([30..55, 65..70]));

            assert_eq!(m.apply_range(VecDeque::from([0..80])), VecDeque::from([0..10, 30..45, 25..30, 20..55, 65..80])); // what if the range to transform spanned all submaps?
            // assert_eq!(m.apply_range(&(0..80)), vec![0..10, 20..45, 50..80 ]); // what if the range to transform spanned all submaps? and then we combined connected ranges and removed overlapped ones?
        }

        #[test]
        fn test_mapping_overlap() {
            let m = Mapping::from(
                vec![
                    SubMapping::from((50, 98, 2)),
                    SubMapping::from((52, 50, 48)),
                ]
                .as_slice(),
            );
            assert_eq!(m.overlap(0..10), false);
            assert_eq!(m.overlap(30..60), true);
        }
    }
}

fn parse_almanac_chunk(chunk: &[&str]) -> Vec<mapping::SubMapping> {
    chunk
        .iter()
        .skip(1)
        .map(|line| {
            mapping::SubMapping::from(
                line.split(" ")
                    .map(str::trim)
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<i64>().expect("parsing map in almanac chunk"))
                    .take(3)
                    .collect_tuple::<(i64, i64, i64)>()
                    .expect("had fewer than 3 map bits"),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_almanac_chunk() {
        let chunk = vec!["soil-to-fertilizer map:", "0 15 37", "37 52 2", "39 0 15"];
        let expected: Vec<mapping::SubMapping> =
            vec![(0, 15, 37).into(), (37, 52, 2).into(), (39, 0, 15).into()];
        assert_eq!(expected, parse_almanac_chunk(&chunk));
    }

    #[test]
    fn test_get_closest_location() {
        let data: &str = "seeds: 79 14 55 13\n\
                          \n\
                          seed-to-soil map:\n\
                          50 98 2\n\
                          52 50 48\n\
                          \n\
                          soil-to-fertilizer map:\n\
                          0 15 37\n\
                          37 52 2\n\
                          39 0 15\n\
                          \n\
                          fertilizer-to-water map:\n\
                          49 53 8\n\
                          0 11 42\n\
                          42 0 7\n\
                          57 7 4\n\
                          \n\
                          water-to-light map:\n\
                          88 18 7\n\
                          18 25 70\n\
                          \n\
                          light-to-temperature map:\n\
                          45 77 23\n\
                          81 45 19\n\
                          68 64 13\n\
                          \n\
                          temperature-to-humidity map:\n\
                          0 69 1\n\
                          1 0 69\n\
                          \n\
                          humidity-to-location map:\n\
                          60 56 37\n\
                          56 93 4";
        // assert_eq!(35, get_closest_location_a(&data));
        assert_eq!(46, get_closest_location_b(&data));
    }
}
