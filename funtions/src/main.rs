fn main() {
    
    let _x : i32 = another_function("first element", "second element");

    println!("value of _x with a i32 {} returned from 'another_funtion()'", _x);
}

// rust uses 'snake_case' and doesn't care where it has been stated but it's important to know whether it is in the same scope
fn another_function(paramenter_1: &str, parameter_2: &str) -> i32 { // -> must state which type you are going to use as parameters
    println!("{} , {}", paramenter_1, parameter_2);

    100 // same as return 100; understanding this as a expression helps
}








