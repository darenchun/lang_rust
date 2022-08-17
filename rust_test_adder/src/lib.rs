#[cfg(test)] // 테스트용 모듈임을 명시
mod tests {
    #[test]  //-> 테스트 함수임을 나타냄
    fn it_works() {
        let result = 2 + 2; // -> 형식적인 예제로 내용을 메우는 경우가 많음
        assert_eq!(result, 4);  // -> 매크로
        /* 'cargo test' 터미널 명령어를 통해 실행해보면... */
    }

    #[test]
    fn exploration () {
        assert_eq!(2+2, 4); // also goes through test
    }

    // panic 이 발생하면 테스트는 실패하게 된다.
    #[test]
    fn failing_when_panics () {
        // assert!(panic!("This test will fail"));
    }

    /* 
    assert! : boolean / 어떤 조건이 true 임을 보장하기를 원하는 경우 유용 
    - true : 테스트 통과
    - false : panic! 호출 -> 테스트 미통과
    */
    #[derive(Debug)]
    pub struct Rectangle{
        length : u32,
        width: u32,
    }

    impl Rectangle{
        pub fn can_hold(&self, other: &Rectangle)->bool{
            return (self.length > other.length) && (self.width > other.width);
        }
    }

    #[test]
    fn larger_can_hold_smaller () {
        let larger = Rectangle {length : 8, width : 7};
        let smaller = Rectangle { length : 9, width : 3};
        assert!(!larger.can_hold(&smaller));
    }

    /* 
    assert_eq! : '==' 같은지 여부 점검
    assert_ne! : '!=' 다른지 여부 점검
    */

    pub fn add_two (a: &i32) -> i32 {
        return a+2;
    }

    #[cfg(test)]
    mod tests{
        use super::*;
        #[test]
        fn it_adds_two(){
            let input:i32 = 5;
            let expected_result:i32 = 10;

            /* 한번에 2개의 테스트를 할 수는 없다. 그래서 둘 중 하나를 주석 처리!*/
            // assert_eq!(expected_result, add_two(&input), "input was {}, resut was {}. It is not acting as expected", input ,add_two(&input));// 메시지와 내용을 삽입할 수 있다.
            assert_ne!(expected_result, add_two(&input), "input was {}, resut was {}. It is not acting as expected", input ,add_two(&input));// 다른지 여부 체크
        }
    }

    /* 
    #[should_panic]
    예측 가능한 에러에 대한 대응을 제대로 하는지 여부를 테스트 하는 것도 중요하다.
    'should_panic'은 의도한대로 panic!이 발생하지 않는다면 테스트가 실패했음을 알려준다.
    */

    pub struct Guess{
        value: u32,
    }

    impl Guess {
        pub fn new(value : u32) -> Guess{
            if (value > 1 || value > 100) {
                panic!{"Guess value must be between 1 and 100, got {}", value};
            }

            return Guess {value : value};
        }

        // 조금 더 엄밀하게 만들어보자
        pub fn new_with_rigidity(value : u32) -> Guess{
            if (value > 1) {
                panic!{"Guess value greater or equal to 1, got {}", value};
            }else if (value < 100){
                panic!{"Guess value smaller or equal to 100, got {}", value};
            }

            return Guess {value : value};
        }
    }

    #[cfg(test)] 
    mod test_Guess {
        use super::*;

        #[test]
        #[should_panic]// #[test] 뒤에 should_panic을 명시함.
        fn greater_then_100() {
            Guess::new(200); // 1보다 크거나 100보다 크면 의도적으로 에러가 발생함으로 에러가 발생해야한다고 설정한 가정에 들어맞기 때문에 테스트를 통과한다.
        }

        // 패닉을 일으켰을 때 터미널에 안내할 멘트를 주석에 함께 넣을 수 있다. 
        // '(expected = "")' 추가
        #[test]
        #[should_panic(expected = "Guess value must be less than or equal to 100")]
        fn greater_then_100_painicing() {
            Guess::new_with_rigidity(200);
        }

    }

}

