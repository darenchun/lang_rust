mod ownership_clone;
mod ownership_methods;
fn main() {
    // 여기까지는 변수를 선언하지 않았으므로 변수가 유효하지 않다.'
    let _s = "literal"; // 여기서부터 유효하다.
    _explanation_of_string();
    run_ownership_method();
} // 여기까지 변수가 유효하다.

// 위 main 에서 대입한 변수에는 literal(소스 코드의 고정된 값을 대표하는 용어)가 사용되어 사실 현실적으로 사용하기는 적합하지 않다. 값이 유동적인 경우가 훨씬 많기 때문이다.
fn _explanation_of_string () {
    let mut _s2: String= String::from("some_other_literals"); // 상기 코드에 사용된 '::'는 method 문법에 따른 것으로 특정 구조체의 종속된 method를 확실하게 표현하기 위한 문법이다. 지금은 그냥 이런게 있다고 생각하고 넘어가자.
    _s2.push_str("+ added literals"); // s2는 String 대표 class에서 실제로 구현된 복사본이라고 볼 수 있으므로 고정 구조체의 '::'를 따르지 않고 간단하게 '.'으로 표현한다.
    println!("{}", &_s2);
    println!("{}", _s2);
}
fn _ownership_1st_move () {
    // 아래 경우는 i32라는 고정된 크기를 가지고 있는 값을 저장하기에 stack에 바로 저장하고 바로 사용할 수 있다. 하지만...
    let _x: i32 = 5;
    let _y: i32 = _x;

    // 이제 String 값에 대한 정의를 살펴보자.
    let _a : String = String::from("hello");
    let _b : String = _a;

    /* 
     일반적인 GC를 가진 언어들의 경우 고정된 값에 대한 포인터를 생성, 반환을 성능을 조금 희생하더라도 자동화하므로 아래코드가 동작하지만, 
    rust에서는 그렇지 않다. 아래 코드는 rust에서는 동작하지 않는다.
    대부분의 언어의 경우 위 연산을 수행하게 되면 같은 값을 참조하는 포인터를 신규 생산하는 소위 '얕은 복사'를 실행하게 되는데,
    rust 에서는 포인터를 새로 생산함과 동시에 이전에 원래 생성된 포인터를 무효화 한다는 차이가 있다.
    */

     //println!("{}", _a); // -> 에러!! 소유권이 '_b' 로 이동한 후에 값에 대한 참조가 소멸한 상태로 해당 변수를 활용을 허용하지 않는다.
     println!("{}", _b);
}


fn run_ownership_method() {
 
}