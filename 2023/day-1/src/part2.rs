use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let result = input
        .split(|b| b == '\n')
        .map(|line| {
            ((0..line.len()).find_map(|i| num(line, i)).unwrap() * 10
                + (0..line.len()).rev().find_map(|i| num(line, i)).unwrap()) as u32
        })
        .sum::<u32>();

    Ok(result.to_string())
}

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[inline(always)]
fn num(line: &str, i: usize) -> Option<usize> {
    let ith_char: char = line.chars().nth(i).unwrap();
    let substr = &line[i..];
    ith_char
        .is_ascii_digit()
        .then_some((ith_char as u8 - b'0') as usize)
        .or_else(|| NUMS
            .iter()
            .enumerate()
            .find(|(_, &name)| substr.starts_with(name))
            .map(|(num, _)| num + 1))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../example2.txt");
        assert_eq!("281", process(input)?);
        Ok(())
    }
}