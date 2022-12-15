// The stack, the heap, and pointers
/* 
● Stack : Very fast, but it requires the size to be precise at compile time. So primitive types(Which compiler already knows) can go on Stack.
● heap : Sometimes variable's type is not decided at the compile/runtime. 
         In this case we you put your data on heap, and register a pointer on stack and look for that pointer to identify which data on heap that you were looking for.
         [you -> pointer(stack) -> actual data(heap)]
*/

fn main() {
    //example
    let my_number = 15; // i32 type
    let single_reference = &my_number; // &i32
    let double_references = &single_reference; // &&i32
    let five_references = &&&&&my_number; // &&&&&i32
    println!("this is : {}", five_references);

}
