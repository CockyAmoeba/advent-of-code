use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let result: usize = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            let draws = parse_game(line);
            (idx + 1, draws )
        })
        .filter(|(_, game_draws)| {
            game_draws.iter()
            .all(|draw| draw[0] <= 12 && draw[1] <= 13 && draw[2] <= 14)
        })
        .map(|(game_id, _)| game_id)
        .sum();

    Ok(result.to_string())
}

fn parse_game(game_line: &str) -> Vec<[u32; 3]> {
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

    draws
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
        let draws = parse_game(input);
        let result = draws.iter().all(|draw| draw[0] <= 12 && draw[1] <= 13 && draw[2] <= 14);
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        assert_eq!("8", process(input)?);
        Ok(())
    }
}