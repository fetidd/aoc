static INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let field = Field(
        INPUT
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let mut s_loc = Loc::default();
    for (row, line) in field.0.iter().enumerate() {
        if let Some(col) = line.iter().position(|x| *x == 'S') {
            s_loc = Loc {
                row,
                col,
                appr: Some(Direction::Up),
            };
            break;
        }
    }
    dbg!(&s_loc);
    let mut corners = vec![];
    let mut a = up(&s_loc);
    if ['J', 'L', '7', 'F'].contains(&field.get(&a)) {
        corners.push(a);
    }
    loop {
        let next_a = dir(&field.get(&a), &a.appr.unwrap());
        if let Ok(next_a) = next_a {
            match next_a {
                Direction::Up => a = up(&a),
                Direction::Down => a = down(&a),
                Direction::Left => a = left(&a),
                Direction::Right => a = right(&a),
            }
        } else {
            panic!();
        }
        if ['J', 'L', '7', 'F'].contains(&field.get(&a)) {
            corners.push(a);
        }
        if field.get(&a) == 'S' {
            corners.push(a);
            break;
        }
    }
    let corners = corners.iter().map(|l| (l.row, l.col)).collect::<Vec<_>>();
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
    println!("{}", (right_sum - left_sum) / 2);
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Field(Vec<Vec<char>>);

impl Field {
    fn get(&self, loc: &Loc) -> char {
        self.0[loc.row][loc.col]
    }
}

/// A point in the field, from top-left.
#[derive(Debug, Clone, Default, Copy)]
struct Loc {
    row: usize,
    col: usize,
    appr: Option<Direction>,
}

impl PartialEq for Loc {
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

fn up(curr: &Loc) -> Loc {
    Loc {
        row: curr.row - 1,
        col: curr.col,
        appr: Some(Direction::Down),
    }
}
fn down(curr: &Loc) -> Loc {
    Loc {
        row: curr.row + 1,
        col: curr.col,
        appr: Some(Direction::Up),
    }
}
fn left(curr: &Loc) -> Loc {
    Loc {
        row: curr.row,
        col: curr.col - 1,
        appr: Some(Direction::Right),
    }
}
fn right(curr: &Loc) -> Loc {
    Loc {
        row: curr.row,
        col: curr.col + 1,
        appr: Some(Direction::Left),
    }
}

static TRAPPED_4_DOTS: &str = "...........\n\
                         .S-------7.\n\
                         .|F-----7|.\n\
                         .||.....||.\n\
                         .||.....||.\n\
                         .|L-7.F-J|.\n\
                         .|..|.|..|.\n\
                         .L--J.L--J.\n\
                         ...........";
static TRAPPED_5_DOTS: &str = "...........\n\
                         .S-------7.\n\
                         .|F-----7|.\n\
                         .||.....||.\n\
                         .||.....||.\n\
                         .|L-7F--J|.\n\
                         .|..||...|.\n\
                         .L--JL---J.\n\
                         ...........";
