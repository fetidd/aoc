pub fn run(input: &str, part: u8) -> String {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("13", &run(input, 1));
    }
}
