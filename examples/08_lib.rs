// Why not anyhow::Result?

mod parser {
    pub fn parse(s: &str) -> Result<u32, String> {
        match s.parse::<u32>() {
            Ok(s) => Ok(s),
            Err(e) => Err(format!("Failed to parse {}: {}", s, e)),
        }
    }
}

fn main() {
    match parser::parse("123a") {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to parse: {}", e)
        }
    }
}
