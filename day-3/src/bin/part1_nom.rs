use day_3::part1_nom::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("{result}");
    Ok(())
}