use crate::custom_error::AocError;

use nom::{
    // character::complete::{alpha0, alphanumeric0, alphanumeric1, digit1, line_ending}, combinator::{iterator, map_opt, opt, ParserIterator}, multi::{many1, separated_list1}, sequence::{preceded, terminated}, IResult
    character::complete::{alpha0, alphanumeric1, digit1, line_ending}, multi::{many1, separated_list1}, sequence::{preceded, terminated}, IResult
};

fn nom_parse_line(line: &str) -> IResult<&str, Vec<&str>> {
    many1(preceded(alpha0, terminated(digit1, alpha0)))(line)
}

fn parse_nom_result(multidigit: Vec<&str>) -> u32 {
    let first = multidigit.first().unwrap().parse::<u32>().unwrap();
    let last = multidigit.last().unwrap().parse::<u32>().unwrap();
    first*10 + last
}

fn parse_lines(input: &str) -> u32 {
    // let mut it: ParserIterator<&str, nom::error::Error<&str>, _>  = iterator(input, terminated(alphanumeric1, line_ending));
    // it.map(|line| {
    //     let parsed = nom_parse_line(line).unwrap().1;
    //     parse_nom_result(parsed)
    // }).sum::<u32>()
    let nom_lines: IResult<&str, Vec<&str>, nom::error::Error<&str>>  = separated_list1(line_ending, alphanumeric1)(input);
    let (_, lines): (&str, Vec<&str>) = nom_lines.unwrap();
    lines.iter().map(|line: &&str| {
        let parsed: Vec<&str> = nom_parse_line(line).unwrap().1;
        parse_nom_result(parsed)
    }).sum::<u32>()
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let result: u32 = parse_lines(input);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;



    #[rstest] 
    #[case("pqr3stu8vwx", vec!["3", "8"], 38)]
    #[case("treb7uchet", vec!["7"], 77)]
    fn nom_test(#[case] input: &str, #[case] interim_val: Vec<&str>, #[case] expected: u32) -> miette::Result<(), nom::Err<nom::error::Error<&'static str>>> {
        let parsed = nom_parse_line(input).unwrap().1;
        assert_eq!(interim_val, parsed);
        let result = parse_nom_result(parsed); 
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        assert_eq!("142", process(input)?);
        Ok(())
    }
}