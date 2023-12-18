use std::sync::Arc;

use itertools::Itertools;
use rayon::prelude::*;

#[cfg(test)]
mod solve {
    use super::*;

    #[test]
    fn solve_a() {
        let data = std::fs::read_to_string("day_05.txt").unwrap();
        assert_eq!(157211394, get_closest_location(&data, parse_seeds));
    }

    #[test]
    fn solve_b() {
        let data = std::fs::read_to_string("day_05.txt").unwrap();
        assert_eq!(157211394, get_closest_location(&data, parse_seed_ranges));
    }
}

fn parse_seeds(seed_line: &str) -> Vec<i64> {
    seed_line
        .split(" ")
        .map(str::parse::<i64>)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect::<Vec<_>>()
}

fn parse_seed_ranges(seed_line: &str) -> Vec<i64> {
    let seed_line: Vec<i64> = seed_line
        .split(" ")
        .map(str::parse::<i64>)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();
    seed_line
        .as_slice()
        .chunks_exact(2)
        .map(|chunk| chunk[0]..chunk[0]+chunk[1])
        .fold(Vec::new(), |mut a, b| {
            a.extend(b); 
            a
        })
}

fn map_seed(mut seed: i64, maps: Vec<Arc<dyn Fn(i64) -> i64 + Send + Sync>>) -> i64 {
    for map_fn in &maps {
        seed = map_fn(seed);
    }
    seed
}

fn get_closest_location(data: &str, seed_parser: fn(&str) -> Vec<i64>) -> i64 {
    let lines = data.lines().collect::<Vec<_>>();
    eprintln!("parsing seeds...");
    let seeds = seed_parser(&lines[0]);
    eprintln!("generating maps...");
    let maps = lines
        .split(|x| x.is_empty())
        .map(|chunk| parse_almanac_chunk(chunk))
        .map(|m| generate_map(&m))
        .collect::<Vec<_>>();
    eprintln!("mapping seeds to locations...");
    seeds
        .par_iter()
        .map(|seed| map_seed(*seed, maps.clone()))
        .reduce(|| i64::MAX, |acc, el| std::cmp::min(acc, el))
}

fn parse_almanac_chunk(chunk: &[&str]) -> Vec<Mapping> {
    chunk
        .iter()
        .skip(1)
        .map(|line| {
            line.split(" ")
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().expect("parsing map in almanac chunk"))
                .take(3)
                .collect_tuple::<Mapping>()
                .expect("had fewer than 3 map bits")
        })
        .collect()
}

type Mapping = (i64, i64, i64); // destination range start, source range start, range length
fn generate_map(mappings: &[Mapping]) -> Arc<dyn Fn(i64) -> i64 + Send + Sync> {
    let mappings = mappings
        .iter()
        .map(|(dest, source, len)| (*source..source + len, dest - source))
        .collect::<Vec<_>>();
    Arc::new(move |source| {
        for (src_range, offset) in mappings.clone().iter() {
            if src_range.contains(&source) {
                return source + offset;
            }
        }
        source
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_almanac_chunk() {
        let chunk = vec!["soil-to-fertilizer map:", "0 15 37", "37 52 2", "39 0 15"];
        let expected = vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)];
        assert_eq!(expected, parse_almanac_chunk(&chunk));
    }

    #[test]
    fn test_generate_map() {
        // seed-to-soil map:
        // 50 98 2
        // 52 50 48
        let mappings = [(50, 98, 2), (52, 50, 48)];
        let map_fn = generate_map(&mappings);
        assert_eq!(0, map_fn(0));
        assert_eq!(1, map_fn(1));
        assert_eq!(49, map_fn(49));
        assert_eq!(52, map_fn(50));
        assert_eq!(53, map_fn(51));
        assert_eq!(99, map_fn(97));
        assert_eq!(50, map_fn(98));
        assert_eq!(51, map_fn(99));
    }

    #[test]
    fn test_get_closest_location() {
        let data = "seeds: 79 14 55 13\n\
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
        assert_eq!(35, get_closest_location(&data, parse_seeds));
        assert_eq!(46, get_closest_location(&data, parse_seed_ranges));
    }
}
