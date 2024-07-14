fn main() {
    let mut original = "String";
    original = "New String";
    let ref_original = &original;
    println!("{}", ref_original);
}
