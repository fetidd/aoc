#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    Working,
    Unknown,
    Damaged,
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '.' => Item::Working,
            '?' => Item::Unknown,
            '#' => Item::Damaged,
            _ => panic!("received bad spring character"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_from_char() {
        assert_eq!(Item::Working, Item::from('.'));
        assert_eq!(Item::Unknown, Item::from('?'));
        assert_eq!(Item::Damaged, Item::from('#'));
    }
}