
/* there are 2ways of using macros.*/
// 1. syntacitc : 의미론적
#[macro_use]
pub mod syntactic_macro;
// 2. precedural : 절차적
// 서서히 1번에 대체되는 중

fn main() {
// 1. syntacitic macro example
let argument = std::env::args().nth(1).expect("Please provide only one argument");
println!("{:?}", factorial!(argument.parse::<u64>().expect("couldn't parse to an integer")));

}
