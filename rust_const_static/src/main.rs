
// const and static
/*
There are two types that don't use let to declare : 
1. const 
2. static
 Also, Rust won't use type inference. You need to write the type for them. Difference between these two are followed as below.
● const is a value that does not change.
● static is a value that does not change and has a fixed memory location.

They are almost the same but mostly Rust programmers use "const".

You write them with "ALL CAPITAL LETTERS", and usually outside of main() since they should live for the whole program.
*/

/*EX 1) outside main(), since it needs to be read across all program*/
const MY_NUMBER: i8 = 8;
//const MY_NUMBER = 8; // -> error! needs to be declared with Type.

/*EX 2) as const it needs to be typed */
static SEASONS :[&str; 4] = ["Spring", "Summer", "Fall", "Winter"];

fn main() {
    
}
