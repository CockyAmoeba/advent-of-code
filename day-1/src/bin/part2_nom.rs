use day_1::part2_nom::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2 with nom")?;
    println!("{result}");
    Ok(())
}