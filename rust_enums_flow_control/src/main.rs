/*
    코드를 사용할 때 열거형이 구조체보다 나은 경우를 생각해봅시다.
    예 : ip 주소 v4 and v6
*/

fn main() {
    enum IpAddrKind {
        V4, // -> these are called 'variant'
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    /* 실제 열거형 인자(variant)를 활용한 함수의 사용 */
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    /* 열거형의 variant를 사용한 함수 */
    fn route(ip_type: IpAddrKind) {}

    /* 통상적인 구조체를 이용한 ip 주소값의 저장*/
    struct IpAddrStruct {
        kind: IpAddrKind,
        address: String,
    }

    // 열거형을 사용한 ip 주소 값의 저장 : v4
    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // 열거형을 사용한 ip 주소 값의 저장 : v6
    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    /* 조금 더 간결하게 이를 나타낼 수 있을까? */
    #[derive(Debug)]
    enum IpAddr {
        V4(String), // 각 variant에 대한 구조체 정의
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1")); // 해당 variant의 타입에 해당하는 값을 바로 할당 후 인스턴스 생성
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    /* 조금 더 깊게... */
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    /*
    위 열거형에는 다른 데이터 타입을 갖는 네 개의 variants 가 있습니다.
    - Quit 은 연관된 데이터가 없다.
    - Move 은 익명 구조체.
    - Write 은 하나의 String.
    - ChangeColor 는 세 개의 i32.

    variants 로 열거형을 정의하는 것은 다른 종류의 구조체들을 정의하는 것과 비슷하다.
    열거형과 다른 점은 struct 키워드를 사용하지 않는다는 것과
    모든 variants 가 Message 타입으로 그룹화된다는 것이다.

    다른 타입을 갖는 여러 개의 구조체를 사용한다면,
    이 메시지 중 어떤 한 가지를 인자로 받는 함수를 정의하기 힘들 것이다.
    Message 열거형은 하나의 타입으로 이것이 가능하다.
    */

    /* 열거형과 구조체의 유사점 : 'impl'의 사용 */
    impl Message {
        // 구조체의 method와 동일하게 열거형도 method를 정의할 수 있다.
        fn call(&self) {}
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    /*
    Rust에는 null 이 없다. Null 자체가 가지는 개념적인 부분에는 문제가 없으나,
    실제 사용 시 null을 null이 아닌 것처럼 사용하려고 할 때 많은 종류의 오류가 발생한다.
    그래서 이를 'Option'이라는 자료형으로 대체하고 있다.
    */

    /* enum Option<T> {
        Some(T),
        None,
    } */

    /*
    'Option<T>' 열거형은 매우 유용하며 기본적으로 포함되어 있기 때문에,
    명시적으로 가져오지 않아도 사용할 수 있다.
    '<T>' 부분은 제네릭이라고 불리는 문법으로 향후 배우게 되겠지만
    현재는 Option 열거형의 variant가 어떤 타입의 데이터라도 가질 수 있다는 것을 의미한다고만 알고 있으면 된다.
    */

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    /*
    간단하게 말하면 Option<T> 와 T(어떤 자료형이든 가능)은 다른 타입이며,
    컴파일러는 Option<T>의 값을 명확한 값이 아니라고 인지한다.

    아래 코드는 컴파일이 되지 않는다.
    */

    /*
       let x : i8 = 10;
       let y: Option<i8> = Some(10);

       let sum = x+y; // ->the trait `Add<main::Option<i8>>` is not implemented for `i8` : 컴파일러가 Option<T>를 어떻게 받아들여야 하는지 몰라 명확한 표기를 요구한다.
    */

    /*
    그럼 Option<T> 타입인 값을 사용할 때,
    Some variant 에서 T 값을 어떻게 가져와서 사용할 수 있을까
    */

    /* match 흐름 제어 연산자 */
    // 일련의 패턴에 대해 어떤 값을 비교한 뒤 어떤 패턴에 매치되었는지를 바탕으로 코드를 수행하도록 해준다.

    /* match를 일종의 동전분류기로 생각하면 쉽다. */
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        //..etc
    }

    fn value_in_cents(coin: Coin) -> u32 {
        // if를 사용한 표현식과 매우 유사하지만, 큰 차이점이 있다:
        // if를 사용하는 경우, 해당 표현식은 boolean을 반환해야하나,
        // 여기서는 어떤 타입이든 가능하다.
        match coin {
            Coin::Penny => {
                // 중괄호로 여러가지 명령들을 묶을 수도 있다.
                println!("Lucky penny!");
                return 1;
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                return 25;
            }
        }
    }

    /* Option<T>를 이용하는 매칭 */
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None, // 기본적으로 option을 이용한 매칭을 활용하는 경우에는 None 을 반환하는 구문을 빼먹을 경우, 컴파일 자체가 되지 않는다.
            Some(i) => {
                println!("There is some value inside : {}", i);
                return Some(i + 1);
            },
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    /* 
    '_' 변경자 
    
    러스트는 우리가 모든 값을 나열하고 싶지 않을 경우에 사용할 수 있는 패턴을 가지고 있다.
    */

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // _ 어떠한 값과도 매칭된다. 우리가 매칭하고 싶은 값의 끝에 이 값을 집어넣음으로써, 매칭하고자 하는 값을 제외한 나머지 값을 모두 매칭시키고 () 단위값(void 빈값 아무것도 안함)에 매칭시킨다.
    }

    /* if_let 문법 */
    /* 매칭시키고자 하는 경우가 딱 하나라면 여러 경우를 나열하기 위한 match를 사용하기엔 과한 느낌이 있다. 이런 경우를 위해 if let 문법이 존재한다. */

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    } // 적을게 너무 많다...
    
    /* if let 문법을 사용하면 이런 형식으로 간단하게 적을 수 있다. */
    if let Some(3) = some_u8_value {
        println!("three");
    }


    /* 
    이번 프로젝트에서는 열거한 값들의 집합 중 하나가 될 수 있는 커스텀 타입을 만들기 위해서 열거형을 사용해보았으며,
    Option<T> 타입이 에러방지를 위해서 어떤 식으로 타입을 이용하는지 예시를 통해 확인하고,
    match나 if let else 문법을 통해서 그 값들을 어떻게 추출하고 사용할 수 있는지 알아보았다.
     */

}
