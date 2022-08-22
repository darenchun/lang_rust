/* 
[unit test]
Convention : 테스트 하고자 하는 모듈에 'test'를 붙여 새로운 파일을 생성하고 테스트를 작성하고 해당 모듈에 '#[cfg(test)]'를 작성 

#[cfg(test)] 
해당 주석이 붙은 모듈의 경우, cli 에서 cargo test 명령어를 통한 컴파일이 이루어졌을 때만 적용될 수 있게 한다.
이후 테스트하고자 하는 함수에는 '#[test]'를 덧붙여 컴파일 될 수 있도록 한다.
*/

// 예시
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/* 
testing private functions 
테스트에서 private function을 테스트할 수 있도록 할 것이냐에 대해서 관련 커뮤니티에서 논의 중이기는 하나, rust 에서는 private function에 대한 테스트를 지원한다.
*/

// 예시
/* testing private fn in public */
pub fn add_two (a: i32) -> i32 {
    return internal_adder(a, 30);
}

fn internal_adder(a:i32 , b:i32)-> i32{
    return a+b;
}

#[cfg(test)]
mod testing_private_funtions {
    use super::*;
    #[test]
    fn private_funtion_test() {
        assert_eq!(4,internal_adder(2,2));
    }
}
