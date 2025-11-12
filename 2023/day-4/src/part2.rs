use std::collections::HashSet;

use crate::custom_error::AocError;

struct Card {
    winning_numbers: HashSet<i64>,
    chosen_numbers: HashSet<i64>,
}

impl Card {
    fn count(&self) -> usize {
        self.winning_numbers
            .intersection(&self.chosen_numbers)
            .count()
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let mut cards: Vec<Card> = Vec::new();

    for line in input.split(|b| b == '\n') {
        let (_, nums) = line.split_once(": ").unwrap();
        let (win, chose) = nums.split_once(" | ").unwrap();

        let winning_numbers = win
            .split_whitespace()
            .map(|snum| snum.parse::<i64>().unwrap())
            .collect::<HashSet<_>>();
        let chosen_numbers = chose
            .split_whitespace()
            .map(|snum| snum.parse::<i64>().unwrap())
            .collect::<HashSet<_>>();
        cards.push(Card {
            winning_numbers,
            chosen_numbers,
        });
    }
    let mut multiplier = vec![1usize; cards.len()];

    for (index, card) in cards.iter().enumerate() {
        let count = card.count();
        for i in index + 1..index + 1 + count {
            multiplier[i] += multiplier[index];
        }
    }
    let result = multiplier.iter().sum::<usize>();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../example2.txt");
        assert_eq!("30", process(input)?);
        Ok(())
    }
}