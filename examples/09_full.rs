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
    // Return with parser::Error on error
    parser::parse("12345")?;

    // Process parsing error with custom error type
    match parser::parse("1234a") {
        Ok(v) => println!("Parsed {}", v),
        Err(parser::Error::Empty) => println!("Failed to parse empty string"),
        Err(parser::Error::ParseError(e)) => println!("Failed to parse: {}", e),
    }

    // Something else that returns Result<T, impl Into<anyhow::Error>>
    std::fs::File::create("foo.txt")?;
    std::env::var("FOO")?;

    Ok(())
}
