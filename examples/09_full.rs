mod parser {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("empty string")]
        Empty,
        #[error("parser error: {0}")]
        ParseError(String),
    }

    pub fn parse(s: &str) -> Result<u32, Error> {
        if s.is_empty() {
            Err(Error::Empty)
        } else {
            match s.parse::<u32>() {
                Ok(s) => Ok(s),
                Err(e) => Err(Error::ParseError(format!("{}", e))),
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    parser::parse("1234a")?;

    std::fs::File::create("foo.txt")?;

    std::env::var("FOO")?;

    Ok(())
}
