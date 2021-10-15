fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    assert_eq!(1, 2);
    debug_assert!(false);
    panic!("ohhh no");
    todo!("here is something missing");
    unimplemented!("here needs something to be done");
}
