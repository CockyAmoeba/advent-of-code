use nom::{character::complete::{digit1, satisfy}, error::ErrorKind, multi::many0, sequence::delimited};
use std::collections::HashSet;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let engine_schematic = parse_schematic(input).unwrap();
    
    let result = engine_schematic.part_numbers.iter()
    .filter(|part_number| {
        part_number.next_to_symbol(&engine_schematic.symbols)
    })
    .map(PartNumber::extract_value)
    .sum::<i64>();

    Ok(result.to_string())
}

fn parse_schematic(input: &str) -> miette::Result<EngineSchematic, AocError> {
    let mut engine_schematic = EngineSchematic {
        part_numbers: Vec::new(),
        symbols: HashSet::new(),
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



    fn extract_value(&self) -> i64 {
        self.value
    }

    fn next_to_symbol(&self, syms: &HashSet<(i64, i64)>) -> bool {
        self.adj_points.intersection(syms).next().is_some()
    }
}

pub struct EngineSchematic {
    part_numbers: Vec<PartNumber>,
    symbols: HashSet<(i64,i64)>,
}

#[cfg(test)]
mod tests {
    use nom::sequence::delimited;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use super::*;

    #[rstest] 
    #[case("......755.+844", vec!["755", "844"], vec![6, 11])]
    #[case("467..114..467", vec!["467", "114", "467"], vec![0, 5, 10])]
    #[case("...*......", vec![], vec![])]
    #[case("..35..633.", vec!["35", "633"], vec![2, 6])]
    #[case("......#...", vec![], vec![])]
    #[case("617*......", vec!["617"], vec![0])]
    #[case(".....+.58.", vec!["58"], vec![7])]
    #[case("..592.....", vec!["592"], vec![2])]
    #[case("......755.", vec!["755"], vec![6])]
    #[case("...$.*....", vec![], vec![])]
    #[case(".664.598..", vec!["664", "598"], vec![1, 5])]
    fn nom_test(#[case] input: &str, #[case] expected: Vec<&str>, #[case] location: Vec<i32>) {
        let result = many0(delimited(many0(satisfy(|c| !c.is_numeric())), digit1::<_,(&str, ErrorKind)>, many0(satisfy(|c| !c.is_numeric()))))(input).unwrap();
        assert_eq!(result.1, expected);
        for (i, exp_val) in expected.iter().enumerate() {
            assert_eq!(result.1[i], *exp_val);
        }
        let mut skip = 0;
        for (i, loc) in location.iter().enumerate() {
            assert_eq!(input[skip..].find(result.1[i]).unwrap() as i32 + skip as i32, *loc);
            skip += 1+*loc as usize;
        }
    }

    #[rstest]
    #[case("...$.*....", vec![3,5])]
    #[case("...*......", vec![3])]
    #[case("..35..633.", vec![])]
    #[case("......#...", vec![6])]
    #[case("617*......", vec![3])]
    #[case(".....+.58.", vec![5])]
    #[case("..592.....", vec![])]
    #[case("......755.", vec![])]
    #[case(".664.598..", vec![])]
    fn test_get_symbols(#[case] input: &str, #[case] expected: Vec<i32>) {
        let mut result = Vec::<i32>::new();
        for (i, c) in input.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                result.push(i as i32);
            }
        }
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_num_hashset() -> miette::Result<()> {
        let pn = PartNumber::new(0 as i64, 0, "467");
        assert_eq!(pn.value, 467);
        let expected = HashSet::from([
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (-1, 2),
            (0, 2),
            (1, 2),
            (-1, 3),
            (0, 3),
            (1, 3),
        ]);
        let diff: HashSet<_> = pn.adj_points.difference(&expected).collect();
        assert_eq!(diff, [].iter().collect());
        let diff2: HashSet<_> = expected.difference(&pn.adj_points).collect();
        assert_eq!(diff2, [].iter().collect());
        assert_eq!(pn.adj_points.len(), 14);

        Ok(())
    }

    #[test]
    fn test_subset_input() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.";
        let engine_schematic = parse_schematic(input).unwrap();
        assert_eq!(input.lines().count(), 3);
        assert_eq!(engine_schematic.symbols.len(), 1);
        assert_eq!(engine_schematic.part_numbers.len(), 4);
        assert_eq!(engine_schematic.part_numbers[0].value, 467);
        assert_eq!(engine_schematic.part_numbers[1].value, 114);
        assert_eq!(engine_schematic.part_numbers[2].value, 35);
        assert_eq!(engine_schematic.part_numbers[3].value, 633);
        assert_eq!(engine_schematic.symbols, HashSet::from([(1, 3)]));
        assert_eq!("502", process(input)?);
        Ok(())
    }

    #[test]
    fn test_partial_input() -> miette::Result<()> {
        let input = "......124..................418.......587......770...........672.................564............................438..........512......653....
665/...*......................*599.....*.983......794*..140..*...........@..963*....................445........*......*.........709.....*...
.......246.....581......701..........108....%.532........../.73..699...927............................*....579.354.464..............298..86.";
        let engine_schematic = parse_schematic(input).unwrap();

        assert_eq!(input.lines().count(), 3);
        assert_eq!(engine_schematic.symbols.len(), 14);
        assert_eq!(engine_schematic.part_numbers.len(), 30);
        assert_eq!(engine_schematic.part_numbers[0].value, 124);
        assert_eq!(engine_schematic.part_numbers[1].value, 418);
        assert_eq!(engine_schematic.part_numbers[2].value, 587);
        assert_eq!(engine_schematic.part_numbers[3].value, 770);
        assert_eq!(engine_schematic.part_numbers[4].value, 672);
        assert_eq!(engine_schematic.part_numbers[5].value, 564);
        assert_eq!(engine_schematic.part_numbers[6].value, 438);
        assert_eq!(engine_schematic.part_numbers[7].value, 512);
        assert_eq!(engine_schematic.part_numbers[8].value, 653);
        assert_eq!(engine_schematic.part_numbers[9].value, 665);
        assert_eq!(engine_schematic.part_numbers[10].value, 599);
        assert_eq!(engine_schematic.part_numbers[11].value, 983);
        assert_eq!(engine_schematic.part_numbers[12].value, 794);
        assert_eq!(engine_schematic.part_numbers[13].value, 140);
        assert_eq!(engine_schematic.part_numbers[14].value, 963);
        assert_eq!(engine_schematic.part_numbers[15].value, 445);
        assert_eq!(engine_schematic.part_numbers[16].value, 709);
        assert_eq!(engine_schematic.part_numbers[17].value, 246);
        assert_eq!(engine_schematic.part_numbers[25].value, 579);
        assert_eq!(engine_schematic.part_numbers[26].value, 354);
        assert_eq!(engine_schematic.part_numbers[27].value, 464);
        assert_eq!(engine_schematic.part_numbers[0].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[1].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[2].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[9].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[10].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[11].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[12].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[13].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[14].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[15].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[16].next_to_symbol(&engine_schematic.symbols), false);

        assert_eq!(engine_schematic.part_numbers[17].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[18].next_to_symbol(&engine_schematic.symbols), false);
        assert_eq!(engine_schematic.part_numbers[19].next_to_symbol(&engine_schematic.symbols), false);
        assert_eq!(engine_schematic.part_numbers[20].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[21].next_to_symbol(&engine_schematic.symbols), false);
        assert_eq!(engine_schematic.part_numbers[22].next_to_symbol(&engine_schematic.symbols), true);
        assert_eq!(engine_schematic.part_numbers[23].next_to_symbol(&engine_schematic.symbols), false);
        assert_eq!(engine_schematic.part_numbers[24].next_to_symbol(&engine_schematic.symbols), true);

        let result = engine_schematic.part_numbers.iter()
            .filter(|part_number| {
            part_number.next_to_symbol(&engine_schematic.symbols)
            })
            .map(PartNumber::extract_value)
            .sum::<i64>();

        assert_eq!(10303, result);

        Ok(())
    }

    #[test]
    fn test_partial_input_2() -> miette::Result<()> {
        let input = ".....32....$.....#...643*..............116........./905......*..../...........311......811$.*........*890..........924..670........=....882.
......*.....81.....*.....636.......317...*...................899.............*....*698............626....................-..+..@.......*....
.......877......256.714...................825.........458....................869..............................54............28.823..110.....";

        let engine_schematic = parse_schematic(input).unwrap();

        assert_eq!(input.lines().count(), 3);
        assert_eq!(engine_schematic.symbols.len(), 19);
        assert_eq!(engine_schematic.part_numbers.len(), 26);

        let result: Vec<i64> = engine_schematic.part_numbers.iter()
            .filter(|part_number| {
            part_number.next_to_symbol(&engine_schematic.symbols)
            })
            .map(PartNumber::extract_value)
            .collect();
        assert_eq!(result.len(), 22);
        assert_eq!(result, [32, 643, 116, 905, 311, 811, 890, 670, 882, 81, 636, 899, 698, 626, 877, 256, 714, 825, 869, 28, 823, 110]);
        Ok(())
    }

    #[test]
    fn test_get_all_symbols() -> miette::Result<()> {
        let input = include_str!("../example1.txt");
        
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}