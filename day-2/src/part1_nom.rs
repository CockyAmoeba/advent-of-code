use crate::custom_error::AocError;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, digit1, line_ending, space1,
    },
    combinator::opt,
    multi::{fold_many1, separated_list1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_, games) = parse(input).expect("should parse");
    let result = games.into_iter().filter(|game| {
        game.draws
            .iter()
            .all(|draw| draw.is_valid())
    }).map(|game| game.id).sum::<u32>();

    Ok(result.to_string())
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    alt((
        tag("red").map(|_| Color::Red),
        tag("green").map(|_| Color::Green),
        tag("blue").map(|_| Color::Blue),
    ))(input)
}

fn cube(input: &str) -> IResult<&str, (u32, Color)> {
    separated_pair(complete::u32, space1, parse_color)(
        input,
    )
}

fn draw(input: &str) -> IResult<&str, Draw> {
    fold_many1(
        terminated(cube, opt(tag(", "))),
        Draw::default,
        |mut draw, (count, color)| {
            match color {
                Color::Red => {
                    draw.red = count;
                }
                Color::Green => {
                    draw.green = count;
                }
                Color::Blue => {
                    draw.blue = count;
                }
            }
            draw
        },
    )(input)
}

pub fn game(input: &str) -> IResult<&str, Game> {
    let (input, id) =
        delimited(tag("Game "), digit1, tag(": "))(input)?;
    let (input, draws) =
        separated_list1(tag("; "), draw)(input)?;
    Ok((input, Game { id: id.parse::<u32>().unwrap(), draws }))
}

pub fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, game)(input)
}

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Draw {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    pub id: u32,
    pub draws: Vec<Draw>,
}

impl Draw {
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
    #[case("blue", Color::Blue)]
    #[case("red", Color::Red)]
    #[case("green", Color::Green)]
    fn test_color_parse(#[case] input: &str, #[case] expected: Color) {
        let (input, result) = parse_color(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(expected, result);
    }

    #[rstest] 
    #[case("3 blue", (3, Color::Blue))]
    #[case("4 red", (4, Color::Red))]
    #[case("2 green", (2, Color::Green))]
    fn test_parse_cube(#[case] input: &str, #[case] expected: (u32, Color)) {
        let (_, result) = cube(input).unwrap();
        assert_eq!(expected, result);
    }

    #[rstest] 
    #[case("1 red, 2 green, 6 blue", Draw { red: 1, green: 2, blue: 6 })]
    #[case("8 green, 6 blue, 20 red", Draw { red: 20, green: 8, blue: 6 })]
    fn test_draw_parse(#[case] input: &str, #[case] expected: Draw) {
        let (_, result) = draw(input).unwrap();
        assert_eq!(expected, result);
    }
    

    #[rstest] 
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", true)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", false)]
    fn test_validate_singel_game(#[case] input: &str, #[case] expected: bool) -> miette::Result<(), nom::Err<nom::error::Error<&'static str>>> {
        let (_, game) = game(input).unwrap();
        assert_eq!(3, game.draws.len());
        assert_eq!(expected, game.draws.iter().all(|draw| draw.is_valid()));
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        assert_eq!("8", process(input)?);
        Ok(())
    }
}