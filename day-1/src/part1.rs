use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let result = input
        .split(|b| b == '\n')
        .map(|line| {
            u32::from(((line.chars().find(|b| b.is_ascii_digit()).unwrap() as u8) - b'0') * 10
                + (line.chars().rev().find(|b| b.is_ascii_digit()).unwrap() as u8)
                - b'0')
        })
        .sum::<u32>();

    //u32::try_from(result).ok()
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        assert_eq!("142", process(input)?);
        Ok(())
    }
}