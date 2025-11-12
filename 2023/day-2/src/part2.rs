use std::cmp;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let result: u32 = input
        .lines()
        .enumerate()
        .map(|(_, line)| {
            let draws = parse_game(line);
            draws
        })
        .map(|draw| {
            draw.iter().product::<u32>()
        })
        .sum();

    Ok(result.to_string())
}

fn parse_game(game_line: &str) -> [u32; 3] {
    let (_, game) = game_line.split_once(": ").unwrap(); 
    // let (_, id) = meta.split_once("Game ").unwrap();
    let draws: Vec<[u32; 3]> = game.split("; ").map(|draw| {
        draw.split(", ").fold([0, 0, 0], |mut acc, item| {
            let (num, color) = item.split_once(' ').unwrap();
            match color {
                "red" => acc[0] += num.parse::<u32>().unwrap(),
                "green" => acc[1] += num.parse::<u32>().unwrap(),
                "blue" => acc[2] += num.parse::<u32>().unwrap(),
                _ => panic!("unexpected color")
            };
            acc
        })
    }).collect();

    let max_draws = draws.iter().fold([0, 0, 0], |mut acc, draw| {
        acc[0] = cmp::max(draw[0], acc[0]);
        acc[1] = cmp::max(draw[1], acc[1]);
        acc[2] = cmp::max(draw[2], acc[2]);
        acc
    });

    max_draws
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use super::*;

    #[rstest] 
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    fn test_validate_singel_game(#[case] input: &str, #[case] expected: u32) -> miette::Result<(), nom::Err<nom::error::Error<&'static str>>> {
        let draws = parse_game(input);
        let result = draws.iter().product();
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