mod ownership_clone {
    // 소유권이라는 문법으로 인해 권한이동이 발생할 경우, 값 자체를 heap 에 복사하여 별개로 관리하고 싶은 경우 clone 을 통해 값을 복사하여 사용할 수 있다.
    pub fn _ownership_clone() {
        let _s1: String = String::from("123");
        let _s2: String = _s1.clone(); //-> clone method 를 통해서 깊은 복사 (포인터 + literal)를 수행하여 별개의 동일한 변수와 값을 생성할 수 있다.

        println!("{},{}", _s1, _s2);
    }

    // stack data 의 복사
    // 특정 type(크기가 고정이라 변동할 가능성이 없는 경우)에는 컴파일러가 clone을 하지 않고도 알아서 값을 복사하여 관리한다
    pub fn _ownership_copy() {
        let _i1: i32 = 1;
        let _i2: i32 = _i1;
        
        // 기존 예시와는 다르게 에러가 발생하지 않는다. primitive 자료형일 경우, 메모리에 할당되는 크기가 정해져있기 때문에 참조를 하는게 아니라 값 자체를 복사해버리는 방식을 택할 수 있다.
        // 이를 Copy Trait라고 한다.
        // Rust 에서 기본적으로 copy trait 가 적용이 되어 있는 자료형
        // 모든 정수형 타입(u32, i32...)
        // bolean 타입
        // char 타입
        // 부동 소수점 타입(f64)
        // 위 타입만을 가지고 있는 튜플 (예 : (i32, i32))
        println!("{},{}", _i1, _i2);
    }
}
