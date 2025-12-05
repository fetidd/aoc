pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let mut iter = self.grid[0].iter();
        for row in self.grid.iter() {
            iter = iter.chain(row.iter());
        }
        iter
    }
}

impl From<&str> for Grid<char> {
    fn from(value: &str) -> Self {
        let grid = value
            .trim()
            .split("\n")
            .map(|row| row.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            height,
            width,
        }
    }
}
