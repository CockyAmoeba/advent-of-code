use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(
            items.next().unwrap().parse::<i32>().unwrap(),
        );
        right.push(
            items.next().unwrap().parse::<i32>().unwrap(),
        );
    }

    left.sort();
    right.sort();

    let result: u32 = std::iter::zip(left, right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
