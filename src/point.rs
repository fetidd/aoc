#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn get_von_neumann_neighbours(&self, max_width: usize, max_height: usize) -> Vec<Point> {
        let mut neighbours = vec![];
        if self.row > 0 {
            neighbours.push((self.row - 1, self.col).into());
        }
        if self.col > 0 {
            neighbours.push((self.row, self.col - 1).into());
        }
        if self.col < max_width - 1 {
            neighbours.push((self.row, self.col + 1).into());
        }
        if self.row < max_height - 1 {
            neighbours.push((self.row + 1, self.col).into());
        }
        neighbours
    }

    pub fn get_moore_neighbours(&self, max_width: usize, max_height: usize) -> Vec<Point> {
        let mut neighbours = self.get_von_neumann_neighbours(max_width, max_height);
        if self.row > 0 {
            if self.col > 0 {
                neighbours.push((self.row - 1, self.col - 1).into());
            }
            if self.col < max_width - 1 {
                neighbours.push((self.row - 1, self.col + 1).into());
            }
        }
        if self.row < max_height - 1 {
            if self.col > 0 {
                neighbours.push((self.row + 1, self.col - 1).into());
            }
            if self.col < max_width - 1 {
                neighbours.push((self.row + 1, self.col + 1).into());
            }
        }
        neighbours
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_get_moore_neighbours() {
    //     let point = Point::from((3, 3));
    //     assert_eq!(vec![Point::from((0_usize, 0_usize))], point.get_moore_neighbours(10, 10));
    // }
}
