use std::io::Write;

static INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let field = Field(
        INPUT
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let mut s_loc = FieldLocation {
        row: 0,
        col: 0,
        appr: Direction::Up,
    };
    for (row, line) in field.0.iter().enumerate() {
        if let Some(col) = line.iter().position(|x| *x == 'S') {
            s_loc = FieldLocation {
                row,
                col,
                appr: Direction::Up,
            };
            break;
        }
    }
    dbg!(&s_loc);
    let mut visited = vec![];
    let mut a = up(&s_loc);
    let mut b = down(&s_loc);
    let mut steps = 2;
    loop {
        let next_a = dir(&field.get(&a), &a.appr);
        if let Ok(next_a) = next_a {
            match next_a {
                Direction::Up => a = up(&a),
                Direction::Down => a = down(&a),
                Direction::Left => a = left(&a),
                Direction::Right => a = right(&a),
            }
        } else {
            println!("A failed with {:?}", &next_a);
            break;
        }
        visited.push(a.clone());
        if a == b {
            println!("{} steps to {:?} {:?}", steps, a, b);
            break;
        }
        let next_b = dir(&field.get(&b), &b.appr);
        if let Ok(next_b) = next_b {
            match next_b {
                Direction::Up => b = up(&b),
                Direction::Down => b = down(&b),
                Direction::Left => b = left(&b),
                Direction::Right => b = right(&b),
            }
        } else {
            println!("B failed with {:?}", &next_b);
            break;
        }
        visited.push(b.clone());
        if a == b {
            println!("{} steps to {:?} {:?}", steps, a, b);
            break;
        }
        steps += 1;
    }
    let mut pipe_loop = field.clone();
    for (row, x) in pipe_loop.0.iter_mut().enumerate() {
        for (col, y) in x.iter_mut().enumerate() {
            if !visited
                .iter()
                .map(|fl| (fl.row, fl.col))
                .any(|x| x == (row, col))
            {
                *y = '.';
            }
        }
    }
    let mut pipe_file = std::fs::File::create("pipe.txt").unwrap();
    for row in pipe_loop.0.into_iter() {
        let line: String = row.iter().fold(String::new(), |mut a, b| {
            a.push(*b);
            a
        });
        pipe_file.write(line.as_bytes()).unwrap();
        pipe_file.write("\n".as_bytes()).unwrap();
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Field(Vec<Vec<char>>);

impl Field {
    fn get(&self, loc: &FieldLocation) -> char {
        self.0[loc.row][loc.col]
    }
}

/// A point in the field, from top-left.
#[derive(Debug, Clone)]
struct FieldLocation {
    row: usize,
    col: usize,
    appr: Direction,
}

impl PartialEq for FieldLocation {
    fn eq(&self, other: &Self) -> bool {
        self.col == other.col && self.row == other.row
    }
}

#[derive(PartialEq, Debug, Clone)]
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

fn up(curr: &FieldLocation) -> FieldLocation {
    FieldLocation {
        row: curr.row - 1,
        col: curr.col,
        appr: Direction::Down,
    }
}
fn down(curr: &FieldLocation) -> FieldLocation {
    FieldLocation {
        row: curr.row + 1,
        col: curr.col,
        appr: Direction::Up,
    }
}
fn left(curr: &FieldLocation) -> FieldLocation {
    FieldLocation {
        row: curr.row,
        col: curr.col - 1,
        appr: Direction::Right,
    }
}
fn right(curr: &FieldLocation) -> FieldLocation {
    FieldLocation {
        row: curr.row,
        col: curr.col + 1,
        appr: Direction::Left,
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
