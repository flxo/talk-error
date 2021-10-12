use anyhow::{Context, Result};

fn main() -> Result<()> {
    let somestring = std::env::args().nth(1).unwrap();
    somestring.parse::<u32>().with_context(|| format!("Failed to parse {}", somestring))?;

    std::fs::File::open("foo").context("Failed to open foo")?;

    Ok(())
}