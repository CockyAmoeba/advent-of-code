use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let result = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let (_, draws) = line.split_once(": ").unwrap();
            let draws = draws.split("; ").map(Draw::new).collect();
            Game { id: idx + 1, draws }
        })
        .filter(|game| {
            game.draws
                .iter()
                .all(|draw| draw.is_valid())
        })
        .map(|game| game.id)
        .sum::<usize>();

    Ok(result.to_string())
}


struct Game {
    id: usize,
    draws: Vec<Draw>,
}

#[derive(Debug, Default)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

impl Draw {
    fn new(s: &str) -> Draw {
        s.split(", ").fold(
            Draw {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut acc, item| {
                let (num, color) = item.split_once(' ').unwrap();
                let num = num.parse::<usize>().unwrap();
                match color {
                    "red" => acc.red = num,
                    "green" => acc.green = num,
                    "blue" => acc.blue = num,
                    _ => panic!("at the disco"),
                };
                acc
            },
        )
    }

    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use super::*;

    #[rstest] 
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", false)]
    fn test_validate_singel_game(#[case] input: &str, #[case] expected: bool) -> miette::Result<(), nom::Err<nom::error::Error<&'static str>>> {
        let (_, draws) = input.split_once(": ").unwrap();
        let draws: Vec<Draw> = draws.split("; ").map(Draw::new).collect();
        assert_eq!(3, draws.len());
        assert_eq!(expected, draws.iter().all(|draw| draw.is_valid()));
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        assert_eq!("8", process(input)?);
        Ok(())
    }
}