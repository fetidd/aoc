#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Spring {
    Working,
    Unknown,
    Damaged,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Spring::Working,
            '?' => Spring::Unknown,
            '#' => Spring::Damaged,
            _ => panic!("received bad spring character"),
        }
    }
}
impl From<&char> for Spring {
    fn from(value: &char) -> Self {
        Self::from(*value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_from_char() {
        assert_eq!(Spring::Working, Spring::from('.'));
        assert_eq!(Spring::Unknown, Spring::from('?'));
        assert_eq!(Spring::Damaged, Spring::from('#'));
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SpringGroup(pub Vec<Spring>);

impl SpringGroup {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<&[char]> for SpringGroup {
    fn from(value: &[char]) -> Self {
        let mut chars = value.iter().peekable();
        let first = chars.peek().expect("no chars to parse!").to_owned();
        let springs = chars
            .take_while(|ch| {
                if *first != '.' {
                    **ch == '#' || **ch == '?'
                } else {
                    false
                }
            })
            .map(Spring::from)
            .collect::<Vec<_>>();
        SpringGroup(springs)
    }
}
