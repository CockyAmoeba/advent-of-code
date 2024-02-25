use std::cmp;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let result: usize = input
        .lines()
        .enumerate()
        .map(|(_, line)| {
            let (_, draws) = line.split_once(": ").unwrap();
            let draws = draws.split("; ").map(Draw::new).collect();
            Game { draws }
        })
        .map(|game| {
            game.get_product()
        })
        .sum();

    Ok(result.to_string())
}

struct Game {
    draws: Vec<Draw>,
}

impl Game {
    fn get_product(&self) -> usize {
        let max_draws = self.draws.iter().fold([0, 0, 0], |mut acc, draw| {
            acc[0] = cmp::max(draw.red, acc[0]);
            acc[1] = cmp::max(draw.green, acc[1]);
            acc[2] = cmp::max(draw.blue, acc[2]);
            acc
        });

        max_draws.iter().product()
    }
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
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use super::*;

    #[rstest] 
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    fn test_validate_singel_game(#[case] input: &str, #[case] expected: usize) -> miette::Result<(), nom::Err<nom::error::Error<&'static str>>> {
        let (_, draws) = input.split_once(": ").unwrap();
        let draws: Vec<Draw> = draws.split("; ").map(Draw::new).collect();
        let game = Game { draws };
        let result = game.get_product();
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}