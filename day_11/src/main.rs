use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

static INPUT: &str = include_str!("../input.txt");

fn main() {
    let universe = Universe::from(INPUT);
    let galaxies = create_galaxies(&universe);
    let steps = find_total_steps(&galaxies, 1000000);
    println!("{}", steps);
}

fn find_total_steps(galaxies: &Vec<Galaxy>, expansion: u64) -> u64 {
    let mut joined: HashMap<(usize, usize), HashSet<(usize, usize)>> =
        HashMap::from_iter(galaxies.iter().map(|g| (g.loc, HashSet::new())));
    let mut steps = 0;
    for galaxy in galaxies.iter() {
        for other in galaxies.iter().filter(|g| g.loc != galaxy.loc) {
            let visited = joined.get(&galaxy.loc).expect("get galaxy visited");
            let visited_other = joined.get(&other.loc).expect("get other visited");
            if !visited.contains(&other.loc) && !visited_other.contains(&galaxy.loc) {
                let v = joined.get_mut(&galaxy.loc).expect("get mut visited");
                v.insert(other.loc);
                steps += find_path(&galaxy, &other, expansion);
            }
        }
    }
    steps
}

#[test]
fn test_find_total_steps() {
    let u = Universe::from(UNEXPANDED);
    let g = create_galaxies(&u);
    let actual = find_total_steps(&g, 10);
    assert_eq!(actual, 1030);
}

fn create_galaxies(universe: &Universe) -> Vec<Galaxy> {
    let galaxies = universe
        .0
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, el)| **el == '#')
                .map(move |(j, _)| universe.spawn_galaxy((i, j)))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    galaxies
}

#[test]
fn test_find_path() {
    let u = Universe::from(UNEXPANDED);
    let g = create_galaxies(&u);
    let actual = find_total_steps(&g, 10);
    assert_eq!(actual, 1030);
    let actual = find_total_steps(&g, 100);
    assert_eq!(actual, 8410);
}

fn find_path(l: &Galaxy, r: &Galaxy, expansion: u64) -> u64 {
    let mut steps = 0u64;
    match l.loc.0.cmp(&r.loc.0) {
        std::cmp::Ordering::Less => {
            let diff = r.loc.0 - l.loc.0;
            for moved in 0..diff {
                if l.universe.get(&(r.loc.0 - moved, r.loc.1)) == 'm' {
                    steps += expansion;
                } else {
                    steps += 1;
                }
            }
        }
        std::cmp::Ordering::Greater => {
            let diff = l.loc.0 - r.loc.0;
            for moved in 0..diff {
                if l.universe.get(&(r.loc.0 + moved, r.loc.1)) == 'm' {
                    steps += expansion;
                } else {
                    steps += 1;
                }
            }
        }
        _ => {}
    }
    match l.loc.1.cmp(&r.loc.1) {
        std::cmp::Ordering::Less => {
            let diff = r.loc.1 - l.loc.1;
            for moved in 0..diff {
                if l.universe.get(&(r.loc.0, r.loc.1 - moved)) == 'm' {
                    steps += expansion;
                } else {
                    steps += 1;
                }
            }
        }
        std::cmp::Ordering::Greater => {
            let diff = l.loc.1 - r.loc.1;
            for moved in 0..diff {
                if l.universe.get(&(r.loc.0, r.loc.1 + moved)) == 'm' {
                    steps += expansion;
                } else {
                    steps += 1;
                }
            }
        }
        _ => {}
    }
    steps
}

static GALAXY_ID: std::sync::RwLock<u16> = std::sync::RwLock::new(0);
#[derive(Debug)]
struct Galaxy<'a> {
    loc: (usize, usize),
    universe: &'a Universe,
}

impl<'a> Galaxy<'a> {
    fn spawn(loc: (usize, usize), universe: &'a Universe) -> Galaxy {
        let new_id = GALAXY_ID.read().unwrap();
        let g = Galaxy { loc, universe };
        drop(new_id);
        let mut new_id = GALAXY_ID.write().unwrap();
        *new_id += 1;
        g
    }
}

fn expand_universe(universe: &Vec<Vec<char>>) -> Universe {
    let mut expanded_indices = vec![];
    for (i, col) in universe[0].iter().enumerate() {
        if *col == '.' {
            if universe.iter().map(|row| row[i]).all(|c| c == '.') {
                expanded_indices.push(i);
            }
        }
    }
    let mut new_u = vec![];
    for row in universe.iter() {
        if row.iter().all(|c| *c == '.') {
            new_u.push(vec!['m'; row.len()]);
        } else {
            let mut new_row = vec![];
            for i in 0..row.len() {
                if expanded_indices.contains(&i) {
                    new_row.push('m');
                } else {
                    new_row.push(row[i]);
                }
            }
            new_u.push(new_row);
        }
    }
    let u = Universe(new_u);
    println!("{}", &u);
    u
}

#[derive(Debug, PartialEq)]
struct Universe(Vec<Vec<char>>);

impl Universe {
    fn from(input: &str) -> Self {
        let new_u = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        expand_universe(&new_u)
    }

    fn get(&self, loc: &(usize, usize)) -> char {
        self.0[loc.0][loc.1]
    }

    fn spawn_galaxy(&self, loc: (usize, usize)) -> Galaxy {
        Galaxy::spawn(loc, self)
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = self
            .0
            .iter()
            .map(|x| {
                x.iter().fold(String::new(), |mut s, c| {
                    s.push(*c);
                    s
                })
            })
            .fold(String::new(), |mut s, l| {
                s.push_str(l.as_str());
                s.push('\n');
                s
            });
        write!(f, "{}", disp)
    }
}

static EXPANDED: &str = "....#........\n\
                         .........#...\n\
                         #............\n\
                         .............\n\
                         .............\n\
                         ........#....\n\
                         .#...........\n\
                         ............#\n\
                         .............\n\
                         .............\n\
                         .........#...\n\
                         #....#.......";

static UNEXPANDED: &str = "...#......\n\
                           .......#..\n\
                           #.........\n\
                           ..........\n\
                           ......#...\n\
                           .#........\n\
                           .........#\n\
                           ..........\n\
                           .......#..\n\
                           #...#.....";

/// A point in the field, from top-left.
#[derive(Debug, Clone, Copy)]
struct Loc<'a> {
    row: usize,
    col: usize,
    field: &'a Universe,
}

impl<'a> Loc<'a> {
    fn up(&mut self) {
        if self.row != 0 {
            self.row -= 1;
        } else {
            panic!("hit the top edge!")
        }
    }
    fn down(&mut self) {
        if self.row + 1 < self.field.0.len() {
            self.row += 1;
        } else {
            panic!("hit the bottom edge!")
        }
    }
    fn left(&mut self) {
        if self.col != 0 {
            self.col -= 1;
        } else {
            panic!("hit the left edge!")
        }
    }
    fn right(&mut self) {
        if self.col + 1 < self.field.0[0].len() {
            self.col += 1;
        } else {
            panic!("hit the right edge!")
        }
    }
}

impl PartialEq for Loc<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.col == other.col && self.row == other.row
    }
}
