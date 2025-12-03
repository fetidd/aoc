pub fn run(input: &str) -> String {
    "2025 3".into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let input = "";
        assert_eq!("", &run(input));
    }
}
