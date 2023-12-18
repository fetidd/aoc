#[derive(Default, Debug, PartialEq)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
    minimum: Hand
}

type Hand = (u32, u32, u32);

fn parse_line(line: &str) -> Game {
    let (id_section, hands) = line.split_once(": ").unwrap();
    let id = id_section[5..].parse::<u32>().expect("failed to parse id");
    let mut hands_vec = vec![];
    let mut minimum = (0, 0, 0);
    for hand in hands.split("; ") {
        let mut out = (0, 0, 0);
        let sections = hand.split(", ").collect::<Vec<&str>>();
        for s in sections {
            if let Some((n, color)) = s.split_once(" ") {
                match color.trim() {
                    "red" => out.0 = {
                        let val = n.parse::<u32>().unwrap();
                        minimum.0 = std::cmp::max(minimum.0, val);
                        val
                    },
                    "green" => out.1 = {
                        let val = n.parse::<u32>().unwrap();
                        minimum.1 = std::cmp::max(minimum.1, val);
                        val
                    },
                    "blue" => out.2 = {
                        let val = n.parse::<u32>().unwrap();
                        minimum.2 = std::cmp::max(minimum.2, val);
                        val
                    },
                    x => panic!("{}?", x),
                }
            }
        }
        hands_vec.push(out);
    }
    Game {
        id,
        hands: hands_vec,
        minimum
    }
}

fn check_hands(hands: &[Hand]) -> bool {
    for hand in hands {
        if hand.0 > 12 || hand.1 > 13 || hand.2 > 14 {
            return false;
        }
    }
    true
}

fn game_power(game: &Game) -> u32 {
    game.minimum.0 * game.minimum.1 * game.minimum.2
}

fn solve_02a(data: &str)  -> u32 {
    let games: Vec<Game> = data.lines().map(parse_line).collect();
    games.into_iter().filter(|game| check_hands(&game.hands)).fold(0u32, |curr, game| curr + game.id)
}

fn solve_02b(data: &str)  -> u32 {
    let games: Vec<Game> = data.lines().map(parse_line).collect();
    games.into_iter().fold(0u32, |curr, game| {
        curr + game_power(&game)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_02a() {
        let data = std::fs::read_to_string("day_02.txt").unwrap();
        assert_eq!(2377, solve_02a(&data));
    }

    #[test]
    fn test_solve_02b() {
        let data = std::fs::read_to_string("day_02.txt").unwrap();
        assert_eq!(71220, solve_02b(&data));
    }

    #[test]
    fn test_game_power() {
        let tests = vec![
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48),
            ("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12),
            ("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560),
            ("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36),
        ];
        for (game, exp) in tests {
            let game = parse_line(game);
            assert_eq!(exp, game_power(&game))
        }
    }

    #[test]
    fn test_parse_line() {
        let tests = vec![
            ("Game 1: 5 red, 6 green; 7 blue, 3 red\n", 1, vec![(5, 6, 0), (3, 0, 7)], (5, 6, 7)),
        ];
        for (game, id, hands, minimum) in tests {
            assert_eq!(
                Game {
                    id,
                    hands,
                    minimum
                },
                parse_line(game)
            );
        }
    }

    #[test]
    fn test_check_hands() {
        let tests = vec![
            (vec![(5, 6, 0), (3, 0, 7)], true),
            (vec![(5, 6, 17), (3, 0, 7)], false),
            (vec![(5, 6, 1), (3, 20, 7)], false),
            (vec![(15, 6, 1), (3, 2, 7)], false),
        ];
        for (hands, exp) in tests {
            assert_eq!(
                exp,
                check_hands(&hands)
            );
        }
    }
}
