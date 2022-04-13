mod ownership_methods{
// Rust 에서는 하단과 같은 개념으로 자바의 '객체' 가 구현된다.
    // struct + impl = "class"
    pub struct OwnerMethods;
    impl OwnerMethods {
        fn main() {
            let cls_01:OwnerMethods = OwnerMethods;
    
            let s = String::from("hello"); // 변수 s 가 범위 내에 생성된다.
            cls_01.takes_ownership(s); // 변수 s의 값이 함수 내로 이동하며 해당 시점부터 위 s는 유효하지 않다.
                                // println!("{}", s);
    
            let x = 5; // 변수 x가 범위 내에 생성된다.
            cls_01.makes_copy(x); // 변수 x 의 값이 함수 내로 이동한다.
                           // 하지만 i32 타입은 복사를 수행하므로, 변수 x는 이후로도 유효하다.
            println!("{}", x);
        }

        fn takes_ownership(&self,some_string: String) {
            // some_string 변수가 범위 내에 생성된다.
            println!("{}", some_string);
        } // 이 지점에서 some_string 변수가 범위를 벗어나게 되어 'drop' 함수가 호출된다. 즉 메모리가 해제된다.

        fn makes_copy(&self,some_integer: i32) {
            // some_integer 변수가 범위 내에 생성된다.
            println!("{}", some_integer);
        } // 이 지점에서 some_integer 변수는 범위를 벗어나지만, copy를 수행하는 type 이므로 'drop' 함수가 호출되지는 않는다.
    }

}
    
