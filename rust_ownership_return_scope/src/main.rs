fn main() {
    /* *************************************** */
    let s1 = gives_ownership(); // gives_ownership 함수의 리턴값이 변수 s1으로 옮겨진다.
    let s2: String = s1; // 변수 s2 가 생성되면서 s1 의 값이 s2로 이동한다. s1에서 drop 함수가 호출된다.
    let s3: String = takes_and_gives_back(s2); // 변수 s2가 takes_and_gives_back 함수의 인자인 a_string 으로 이동한 후 drop 함수 호출

    //println!("{}", s1); //소유권 소실로 인한 drop 함수 호출로 더 이상 유효한 함수가 되지 않는다
    //println!("{}", s2); //소유권 소실로 인한 drop 함수 호출로 더 이상 유효한 함수가 되지 않는다
    println!("{}", s3); //최종적으로 소유권을 획득한 s3는 유효하다.
                        /* *************************************** */
    /*
     변수의 소유권은 매번 같은 패턴을 따른다. 즉, 값을 다른 변수에 할당하면 소유권이 옮겨진다.
    힙 메모리에 저장된 변수의 데이터는 소유권이 다른 변수로 옮겨지지 않았다면, 범위를 벗어날 때, drop 함수에 의해 범위의 끝에서 제거된다.
     모든 함수가 소유권을 확보하고다시 리턴하는 방식의 동작은 다소 거추정스럽다. 함수에 갑을 전달할 때 소유권은 이전하지 않고 싶다면 어떻게 해야할까?
    함수에 전달한 변수를 나중에 다시 사용하기 위해서 매번 그 변숫값을 함수의 실제 리턴값과 함꼐 다시 해야한다면 엄청난 노동이 될 것이다.

     하지만 다행히도 rust는 참조의 개념을 사용할 수 있다.
     */

    let s1 = String::from("literal");
    let length = calculate_length(&s1);

    println!("{} 의 길이는 {} 입니다.", s1, length);

    /*
     calculate_length 함수의 선언부에서는 String 대신 &String 을 사용하여
    소유권을 가지고 오지 않고도 값을 참조(reference)할 수 있다.

    [* note : 참조의 반대는 '역참조'라고 하며 '*'라는 역참조 연산자를 이용한다. ]

     이처럼 함수 매개변수로 참조(reference)를 전달하는 것을 '대여(borrowing)'이라고 한다.
    물론 실제 값을 '참조'만 하는 만큼 그 값을 변동한다고 해서 원래 값을 변동할 수 있는 것은 아니다.
    변수가 기본적으로 불변인 것처럼 참조로 넘겨주는 값(변수)도 역시 기본은 불변이다.
    */

    /* 
     하지만 아래의 예와 같이 예외적으로 하나의 범위 내에서 딱 한 개의 '가변 참조'를 인자로 넘겨주어 함수 상에서 원래 값에 변동을 줄 수도 있다.
    
     */
    { // 하나의 범위 내에서 원래의 값을 변동할 수 있도록 mut 변수를 생성해두고 가변 참조를 change 함수에 넘겨주어 원래 s 값에 변동을 줄 수있다.
     let mut s : String  = String::from("literal");
     println!("원래 s의 값 : {}" ,s);
     change(&mut s);
     println!("가변 참조로 변동 준 s의 값 : {}" , s);
     //let s2:String = change(&mut s2); // 하나의 가변 참조를 이미 사용하고 난 후 2번째 사용을 허용하지 않는다. 이를 방지하는 이유는 컴파일 시 데이터 경합을 방지하기 위해서이다.
    }

    {
              /*
      let mut s = String::from("mouse");
      let r1 = &s; // 문제 없음
      let r2 = &s; // 문제 없음
 
      let r3 = &mut s;
      println!("{},{} and {}" , r1,r2,r3);
      cannot borrow `s` as mutable because it is also borrowed as immutable mutable borrow occurs here
      이미 불변참조(일반 참조)로 원본 변수가 참조를 당하고 있는 입장이라면 값이 변화를 방지해야하기 떄문에 이 또한 에러가 발생한다.
      */
    }

    /* 
     죽은 참조
     죽은 포인터란, 이미 해제되어 다른 정보를 저장하도록 변경된 메모리를 계속해서 참조하는 포인터를 의미한다. 이 포인터를 참조하는 것을 죽은 참조라고 한다.
     rust는 이 것을 방지하기 위해 어떤 데이터에 대한 참조를 생성하면 컴파일러가 해당 데이터에 대한 참조를 실행하기에 앞서 데이터가 범위를 벗어나지 않았는지를 확인해준다.
    */
    {
    /*  아래 함수 dangle 에서는 함수 내에서 생성된 String 타입의 변수 s의 수명이 dangle을 벗어나는 순간 소멸하기 때문에 
        이를 참조하는 &s 또한 유효하지 않은 값을 참조하게 되어 값을 할당할 수 없는 상태에 놓이게 된다. 
        let reference_to_nothing = dangle();
    */
    }

    /* 
    [정리]
    참조는 아래 2가지 조건을 무조건 만족해야 한다.
    1. 어느 한 시점에서 코드는 하나의 가변 참조 또는 여러 개의 불변 참조를 생성할 수 있으나 동시에 2가지 모두 생성할 수는 없다.
    2. 참조는 항상 유효해야 한다. (이미 소멸되어 유효하지 않은 값에 대한 포인터의 참조를 금지해야 한다.)
    */
}

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // gives_ownership 함수의 리턴값은 호출한 함수로 옮겨진다.

    return some_string; // 변수가 새로 생성된다.
}

// 아래 함수는 String 인자를 받아 그 값을 다시 리턴한다.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 변수가 해당 함수 범위 내에 생성된다.
    return a_string; // 변수 a_string을 리턴하면 그 값이 후출한 함수로 옯겨진다.
}

fn calculate_length(s: &str) -> usize {
    // 매개 변수 &s 는 할당하게 되는 String 변수의 포인터(참조값)이다.
    return s.len();
} // 이 시점에서 변수 s는 범위를 벗어난다. 하지만 이 변수는 자신이 가리키는 값에 대한 소유권을 가지고 있는 것이 아니기 때문에 아무런 일도 발생하지 않는다.

fn change(some_string: &mut String) {
    some_string.push_str(" + literal");
}

/* fn dangle() -> &String {
    let s = String::from("literal");
    return &s;
} */
