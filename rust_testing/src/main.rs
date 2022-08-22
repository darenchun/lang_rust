/*
[Testing : 기대한 대로 동작하는지 테스팅 하는 함수 ]
1. 필요한 데이터 혹은 상태를 설정
2. 우리가 테스트하고 싶은 코드 실행
3. 결과가 예상대로인지 단언하기(assert)

간단하게 러스트에서 테스트란, test 속성(attribute)이 주석으로 달려진(annotated) 함수를 지칭한다.
※ attribute : 코드 조각에 대한 메타데이터(ex : #[derive])

[테스팅 방법]
1. 'fn' 위에 #[test]를 작성한다.
2. cargo test를 통해 위 주석이 달려있는 함수들을 실행
3. 'bin' 파일로 실행결과가 도출

Rust 에서는 2가지 테스팅 방식을 기반으로 지원이 이루어지고 있다.
1. unit test : 단일 모듈을 독립적으로 테스트 / 최소 단위 + private interface -> go to unit_test.rs
2. integration test : 현재 프로젝트의 단위에서 벗어나 있다는 것을 기반으로 한 테스팅
*/

mod unit_test;

fn main() {
}

#[cfg(test)]
mod test_01 {
    #[test]
    fn test_addition(){
        let a: u32 = 0;
        let b: u32 = 12;
        let c: &u32 = &a;
        let d: &u32 = &b;

        let e = (a+b) == (*c+*d);
        println!("c+d :  {}", c+d);
        println!("&(c+d) :  {}", &(c+d));
        println!("(*c+*d) :  {}", (*c+*d));
        println!("(a+b) :  {}", a+b);
        println!("is a+b and De-referenced a + De-referenced b same? {}", e);

        assert_eq!(e, true);
    }
}


pub mod module_for_testing{
    fn it_adds_two (param_for_adding:i32) -> i32 {
        param_for_adding + 2
    }   
}