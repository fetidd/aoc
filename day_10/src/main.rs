static INPUT: &'static str = include_str!("../input.txt");
static TRAPPED_4_DOTS: &str = "...........\n\
                               .F-------7.\n\
                               .|F-----7|.\n\
                               .||.....||.\n\
                               .||.....||.\n\
                               .|L-7.F-J|.\n\
                               .|..|.|..|.\n\
                               .S--J.L--J.\n\
                               ...........";

static TRAPPED_5_DOTS: &str = "...........\n\
                               .F-------7.\n\
                               .|F-----7|.\n\
                               .||.....||.\n\
                               .||.....||.\n\
                               .|L-7F--J|.\n\
                               .|..||...|.\n\
                               .S--JL---J.\n\
                               ...........";

static SQUARE: &str = ".........\n\
                       ..F---7..\n\
                       ..|...|..\n\
                       ..|...|..\n\
                       ..|...|..\n\
                       ..S---J..\n\
                       .........";

#[test]
fn test_find_area() {
    let field = Field::new(SQUARE);
    let pipe = field.find_pipe();
    let corners = pipe.iter().cloned().filter(|tile| !['-', '|'].contains(&field.get(tile))).collect::<Vec<_>>();
    let total_area = find_area_shoelace(&corners);
    let area_inside = find_area_picks(pipe.len() as i32, total_area);
    assert_eq!(9, area_inside);
}

#[test]
fn test_trapped_4_dots() {
    let field = Field::new(TRAPPED_4_DOTS);
    let pipe = field.find_pipe();
    let corners = pipe.iter().cloned().filter(|tile| !['-', '|'].contains(&field.get(tile))).collect::<Vec<_>>();
    let area_shoelace = find_area_shoelace(&corners);
    let area_inside = find_area_picks(pipe.len() as i32, area_shoelace);
    assert_eq!(4, area_inside);
}

#[test]
fn test_trapped_5_dots() {
    let field = Field::new(TRAPPED_5_DOTS);
    let pipe = field.find_pipe();
    let corners = pipe.iter().cloned().filter(|tile| !['-', '|'].contains(&field.get(tile))).collect::<Vec<_>>();
    let area_shoelace = find_area_shoelace(&corners);
    let area_inside = find_area_picks(pipe.len() as i32, area_shoelace);
    assert_eq!(5, area_inside);
}

fn main() {
    let field = Field::new(INPUT);
    let pipe = field.find_pipe();
    let corners = pipe.iter().cloned().filter(|tile| !['-', '|'].contains(&field.get(tile))).collect::<Vec<_>>();
    let area_shoelace = find_area_shoelace(&corners);
    let area_inside = find_area_picks(pipe.len() as i32, area_shoelace);
    println!("{}", area_inside);
}

fn find_area_picks(outer_points: i32, total_area: i32) -> i32 {
    // pick's theorem: Area = inner_points + (outer_points / 2) - 1
    // rearrange to get inner_points: inner_points = Area - (outer_points / 2) + 1
    total_area + 1 - (outer_points / 2)
}

fn find_area_shoelace(corners: &Vec<(i32, i32)>) -> i32 {
    let mut left_sum = 0;
    let mut right_sum = 0;
    for i in 0..corners.len() {
        let l = corners[i].0;
        let mut r_i = i + 1;
        if r_i == corners.len() {
            r_i = 0;
        }
        let r = corners[r_i].1;
        left_sum += l * r;
    }
    for i in 0..corners.len() {
        let l = corners[i].1;
        let mut r_i = i + 1;
        if r_i == corners.len() {
            r_i = 0;
        }
        let r = corners[r_i].0;
        right_sum += l * r;
    }
    (((right_sum - left_sum) / 2) as i32).abs()
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Field(Vec<Vec<char>>);

impl Field {
    fn new(input: &str) -> Self {
        Self(input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>())
    }

    fn get(&self, loc: &(i32, i32)) -> char {
        self.0[loc.0 as usize][loc.1 as usize]
    }

    fn spawn_loc(&self) -> Loc {
        let mut s_loc = Loc {
            row: 0,
            col: 0,
            appr: None,
            field: self
        };
        for (row, line) in self.0.iter().enumerate() {
            if let Some(col) = line.iter().position(|x| *x == 'S') {
                s_loc.row = row;
                s_loc.col = col;
                break;
            }
        }
        s_loc
    }

    fn find_pipe(&self) -> Vec<(i32, i32)> {
        let mut cursor = self.spawn_loc();
        let mut pipe = vec![];
        cursor.up();
        pipe.push((cursor.row as i32, cursor.col as i32));
        loop {
            let next_a = dir(&self.get(&(cursor.row as i32, cursor.col as i32)), &cursor.appr.unwrap());
            if let Ok(next_a) = next_a {
                match next_a {
                    Direction::Up => cursor.up(),
                    Direction::Down => cursor.down(),
                    Direction::Left => cursor.left(),
                    Direction::Right => cursor.right(),
                }
            } else {
                panic!()
            }
            pipe.push((cursor.row as i32, cursor.col as i32));
            if self.get(&(cursor.row as i32, cursor.col as i32)) == 'S' {
                return pipe;
            }
        }
    }
}

/// A point in the field, from top-left.
#[derive(Debug, Clone, Copy)]
struct Loc<'a> {
    row: usize,
    col: usize,
    appr: Option<Direction>,
    field: &'a Field
}

impl<'a> Loc<'a> {
    fn up(&mut self) {
        if self.row != 0 {
            self.row -= 1;
            self.appr = Some(Direction::Down);
        } else {
            panic!("hit the top edge!")
        }
    }
    fn down(&mut self) {
        if self.row + 1 < self.field.0.len() {
            self.row += 1;
            self.appr = Some(Direction::Up);
        } else {
            panic!("hit the bottom edge!")
        }
    }
    fn left(&mut self) {
        if self.col != 0 {
            self.col -= 1;
            self.appr = Some(Direction::Right);
        } else {
            panic!("hit the left edge!")
        }
    }
    fn right(&mut self) {
        if self.col + 1 < self.field.0[0].len() {
            self.col += 1;
            self.appr = Some(Direction::Left);
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

#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn dir(pipe: &char, approach_dir: &Direction) -> Result<Direction, String> {
    match (*pipe, approach_dir) {
        ('F', Direction::Right) => Ok(Direction::Down),
        ('F', Direction::Down) => Ok(Direction::Right),
        ('J', Direction::Up) => Ok(Direction::Left),
        ('J', Direction::Left) => Ok(Direction::Up),
        ('L', Direction::Right) => Ok(Direction::Up),
        ('L', Direction::Up) => Ok(Direction::Right),
        ('7', Direction::Left) => Ok(Direction::Down),
        ('7', Direction::Down) => Ok(Direction::Left),
        ('|', Direction::Up) => Ok(Direction::Down),
        ('|', Direction::Down) => Ok(Direction::Up),
        ('-', Direction::Left) => Ok(Direction::Right),
        ('-', Direction::Right) => Ok(Direction::Left),
        _ => Err(format!("{} {:?}", &pipe, &approach_dir)),
    }
}




