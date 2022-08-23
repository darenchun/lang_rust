/* 
[integration test]
Convention in integration test, you need to think your code as an 'external library', and suppose your 'test module' isolated from your code.
This case we create a folder called 'test' and create an module isolated from your code and write test cases there.
'Cargo' will make each of the files as an individual case.
 */
 use rust_testing;
mod common;
#[test]
fn it_adds_two(){
    common::setup();
    assert_eq!(4, rust_testing::add_two(2));
}
