use std::cmp;

use nom::{branch::alt, bytes::complete::tag, character::complete::{self, digit1, line_ending, space1}, combinator::opt, multi::{fold_many1, separated_list1}, sequence::{delimited, separated_pair, terminated}, IResult, Parser};

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let (_, games) = parse(input).expect("should parse");
    let result: u32 = games.into_iter()
        .map(|game| {
            game.get_product()
        })
        .sum();

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
    let (input, _) =
        delimited(tag("Game "), digit1, tag(": "))(input)?;
    let (input, draws) =
        separated_list1(tag("; "), draw)(input)?;
    Ok((input, Game { draws }))
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

pub struct Game {
    draws: Vec<Draw>,
}

impl Game {
    fn get_product(&self) -> u32 {
        let max_draws = self.draws.iter().fold([0, 0, 0], |mut acc, draw| {
            acc[0] = cmp::max(draw.red, acc[0]);
            acc[1] = cmp::max(draw.green, acc[1]);
            acc[2] = cmp::max(draw.blue, acc[2]);
            acc
        });

        max_draws.iter().product()
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
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
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    fn test_validate_singel_game(#[case] input: &str, #[case] expected: u32) -> miette::Result<(), nom::Err<nom::error::Error<&'static str>>> {
        let (_, game) = game(input).unwrap();
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