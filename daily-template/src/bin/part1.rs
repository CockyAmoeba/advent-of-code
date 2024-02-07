use {{crate_name}}::part1::process;
use miette::Context;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    let result = process(file).context("process part 1")?;
    println!("{result}");
    Ok(())
}