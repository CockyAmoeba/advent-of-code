use nom::{character::complete::{digit1, satisfy}, error::ErrorKind, multi::many0, sequence::delimited};
use std::collections::HashSet;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let engine_schematic = parse_schematic(input).unwrap();

    let mut total = 0;
    'next_gear: for gear in &engine_schematic.gears {
        let mut matches = Vec::new();
        for part_number in engine_schematic.part_numbers.iter() {
            if part_number.adj_points.contains(&gear) {
                if matches.len() == 2 {
                    continue 'next_gear;
                }
                matches.push(part_number.value);
            }
        }
        if matches.len() == 2 {
            total += matches[0] * matches[1];
        }
    }

    Ok(total.to_string())
}

fn parse_schematic(input: &str) -> miette::Result<EngineSchematic, AocError> {
    let mut engine_schematic = EngineSchematic {
        part_numbers: Vec::new(),
        symbols: HashSet::new(),
        gears: HashSet::new(),
    };
    for (idx, line) in input.lines().enumerate() {
        let (_, nums) = many0(delimited(many0(satisfy(|c| !c.is_ascii_digit())), digit1::<_,(&str, ErrorKind)>, many0(satisfy(|c| !c.is_ascii_digit()))))(line).unwrap();
        let mut processed_idx = 0;
        for num in nums.iter() {
            let loc = line[processed_idx..].find(num).unwrap() as i64;
            engine_schematic.part_numbers.push(PartNumber::new(idx as i64, loc + processed_idx as i64, num));
            processed_idx += 1+loc as usize;
        }
        for (i, c) in line.chars().enumerate() {
            if !c.is_ascii_digit() && c != '.' {
                engine_schematic.symbols.insert((idx as i64, i as i64));
            }
            if !c.is_ascii_digit() && c == '*' {
                engine_schematic.gears.insert((idx as i64, i as i64));
            }
        }            
    }
    Ok(engine_schematic)
}


#[derive(Debug)]
struct PartNumber {
    value: i64,
    adj_points: HashSet<(i64, i64)>,
}

impl PartNumber {
    fn new(row: i64, col: i64, part_number: &str) -> Self {
        let mut points = HashSet::from([
            (row - 1, col - 1),
            (row, col - 1),
            (row + 1, col - 1), // left hand side
            (row - 1, col),
            (row + 1, col), // above and below
            (row - 1, col + 1),
            (row, col + 1),
            (row + 1, col + 1), // right hand side
        ]);
        for adj_col_index in 1..part_number.len() {
            points
            .extend(
                [
                    (row - 1, col + 1 + adj_col_index as i64), 
                    (row, col + 1 + adj_col_index as i64), 
                    (row + 1, col + 1 + adj_col_index as i64),
                ]
            );
        }
        match part_number.parse::<i64>() {
            Ok(value) => Self {
                value,
                adj_points: points,
            },
            Err(_) => Self {
                value: 0,
                adj_points: points,
            }
        }
    }
}

pub struct EngineSchematic {
    part_numbers: Vec<PartNumber>,
    symbols: HashSet<(i64,i64)>,
    gears: HashSet<(i64, i64)>,
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../example2.txt");
        assert_eq!("467835", process(input)?);
        Ok(())
    }
}