/*

https://doc.rust-lang.org/std/result/enum.Result.html#
https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap
https://doc.rust-lang.org/std/result/enum.Result.html#method.expect

*/

fn main() {
    std::net::TcpStream::connect("localhost:12345").unwrap();
    std::net::TcpStream::connect("localhost:12345").expect("Failed to connect");
}
