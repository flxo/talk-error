fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    // std::env::set_var("RUST_BACKTRACE", "full");

    assert_eq!(1, 2);

    debug_assert!(false);

    todo!("here is something missing");
    unimplemented!("here needs something to be done");

    panic!("ohhh no");

}
