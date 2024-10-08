use day_5::part2::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("{result}");
    Ok(())
}