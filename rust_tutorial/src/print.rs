pub fn run() {
    // Print to console
    println!("Hello rs file");

    // print raw literal --> bumps out error since we aren't using formatting for printing
    // println!(11);

    // Using formatting for placeholders
    println!("{} for everyone, who wants fast execution {}","Rust", "speed");

    // Named arguments
    println!("{name_one} , {name_two} ...",name_one = "이름 1",  name_two = "이름 2",);

    // placeholder traits -> you can do simple expressions too!!
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // basic math
    println!("10 + 10 = {}", 10 + 10);
}