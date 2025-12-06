use std::ops::{Index, IndexMut};

use crate::point::Point;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn get_point_from_index(&self, index: usize) -> Point {
        Point::from((index / self.width, index % self.width))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        let index = self.width * row + col;
        &self.data[index]
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        let index = self.width * index.row + index.col;
        &self.data[index]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        let index = self.width * index.row + index.col;
        &mut self.data[index]
    }
}

impl From<&str> for Grid<char> {
    fn from(value: &str) -> Self {
        assert!(value.len() > 4); // not really a grid with less than this 
        let mut data = vec![];
        let mut width = 0;
        let mut height = 1;
        for (i, ch) in value.trim().chars().enumerate() {
            if ch == '\n' {
                if width == 0 {
                    width = i;
                }
                height += 1;
            } else {
                data.push(ch);
            }
        }
        assert_eq!(width * height, data.len());
        Self {
            data,
            height,
            width,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;

    use super::*;

    #[test]
    fn test_indexing_tuple() {
        let grid_input = "123\n456\n789\n";
        let grid: Grid<char> = grid_input.into();
        assert_eq!(grid[(0, 0)], '1');
        assert_eq!(grid[(0, 1)], '2');
        assert_eq!(grid[(0, 2)], '3');
        assert_eq!(grid[(1, 0)], '4');
        assert_eq!(grid[(1, 1)], '5');
        assert_eq!(grid[(1, 2)], '6');
        assert_eq!(grid[(2, 0)], '7');
        assert_eq!(grid[(2, 1)], '8');
        assert_eq!(grid[(2, 2)], '9');
    }

    #[test]
    fn test_indexing_point() {
        let grid_input = "123\n456\n789\n";
        let grid: Grid<char> = grid_input.into();
        assert_eq!(grid[Point::from((0, 0))], '1');
        assert_eq!(grid[Point::from((0, 1))], '2');
        assert_eq!(grid[Point::from((0, 2))], '3');
        assert_eq!(grid[Point::from((1, 0))], '4');
        assert_eq!(grid[Point::from((1, 1))], '5');
        assert_eq!(grid[Point::from((1, 2))], '6');
        assert_eq!(grid[Point::from((2, 0))], '7');
        assert_eq!(grid[Point::from((2, 1))], '8');
        assert_eq!(grid[Point::from((2, 2))], '9');
    }
}
