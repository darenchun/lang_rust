fn main() {
    // mutable variables : allowing to reassign(mutable) the literal of the varialble
    let mut x = 5;
    println!("value of x = {}", x);
    x = 6; // it's okay!!
    println!("value of x = {}", x);

    // shadowing variables : reallocating the variable while keeping the trait of nun-mutability of variable
    let x = 10;
    // x = 12 => //  error!!
    let x = x + 1;
    let x = x * 2;
    println!("value of x = {}", x);

    // other example of shadowing
    let spaces = "     ";
    println!("value of spaces = {}", spaces);
    let spaces = spaces.len();
    println!("value of spaces = {}", spaces);
    let spaces = "    ";

    // spaces = spaces.len(); // error since string cannot be equal to integer;
    println!("value of spaces = {}", spaces);

    // tuples & array
    let tuple_1 : (i32, f64, u8);
    tuple_1 = (500, 6.4, 1);
    println!("value of 1st element of tuple_1 = {}", tuple_1.0); // -> variable.# : format for selecting leterals of index
    println!("value of 2nd element of tuple_1 = {}", tuple_1.1);
    println!("value of 3rd element of tuple_1 = {}", tuple_1.2);

    let tuple_2 = (500, 9.0, 1); // Can also use auto type format from compilers and assigning variables at once
    let (a, b, c) = tuple_2; // Can also allocate all tuple elements at once
    println!("value of a,b,c = {},{},{}", a,b,c); // printing is also possible

    let array_1 = [1,2,3,4,5,6];
    println!("value of 1st element of array_1 = {}", array_1[0]); // -> variable[#] : format for selecting leterals of index
    println!("value of 2nd element of array_1 = {}", array_1[1]); // -> variable[#] : format for selecting leterals of index
    println!("value of 3rd element of array_1 = {}", array_1[2]); // -> variable[#] : format for selecting leterals of index

    // creating runtime error
    for i in 0..array_1.len() + 1 {
        println!("whether has runtime error = {}", array_1[i]); //-> should have error since it looks for additional element of 'array_1'
    }

    
}

