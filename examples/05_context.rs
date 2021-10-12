// The most annoying error message ever!

fn main() -> Result<(), std::io::Error> {
    std::fs::File::open("foo")?;

    Ok(())
}