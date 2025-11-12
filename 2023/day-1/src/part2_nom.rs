use nom::{branch::alt, bytes::complete::tag, character::complete::{alphanumeric1, anychar, line_ending}, combinator::{iterator, value}, multi::separated_list1, IResult};
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let result: u32 = parse_lines(input);
    Ok(result.to_string())
}

fn parse_lines(input: &str) -> u32 {
    let nom_lines: IResult<&str, Vec<&str>, nom::error::Error<&str>>  = separated_list1(line_ending, alphanumeric1)(input);
    let (_, lines): (&str, Vec<&str>) = nom_lines.unwrap();
    lines.iter().map(|line: &&str| {
        parse_line(line)
    }).sum::<u32>()
}

fn parse_line(line: &&str) -> u32 {
    let parsed: (&str, Vec<u32>) = parser(line).unwrap();
    let mut it = parsed.1.iter();
    let first = it.next().expect("should be a number");

    match it.last() {
        Some(num) => first * 10 + num,
        None => first * 10 + first,
    }
}

fn parser(input: &str) -> IResult<&str, Vec<u32>> {
    let mut it = iterator(input, numbers);
    let output: Vec<u32> = it.flatten().collect();
    let (input, _) = it.finish()?;
    Ok((input, output))
}

fn numbers(input: &str) -> IResult<&str, Option<u32>> {
    let res: IResult<&str, u32> = alt((
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input);

    let (input, digit) = anychar(input)?;

    match res {
        Ok((_, digit)) => Ok((input, Some(digit))),
        Err(_) => Ok((input, digit.to_digit(10))),
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn test_process() -> miette::Result<(), nom::Err<nom::error::Error<&'static str>>> {
        let input = include_str!("../example2.txt");
        assert_eq!("281", process(input).unwrap());
        Ok(())
    }
}