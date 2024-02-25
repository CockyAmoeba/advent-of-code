use day_2::part2_struct::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file).context("process part 2 struct")?;
    println!("{result}");
    Ok(())
}