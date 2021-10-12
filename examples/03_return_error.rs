use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut stream = std::net::TcpStream::connect("localhost:1234")?;

    let mut buffer = [0u8; 4];
    stream.read_exact(&mut buffer)?;

    Ok(())
}
