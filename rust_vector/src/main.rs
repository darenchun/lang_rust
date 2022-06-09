fn main() {
    // Vec<T> 벡터는 단일 데이터 구조안에 같은 타입의 데이터를 저장할 수 있도록 해주는 하나의 타입입니다.

    let v: Vec<i32> = Vec::new(); // 타입이 'i32'로 명시된 것을 주목하자. 벡터는 제네릭을 이용하여 구현되었다.

    /*
    일반적으로 타입을 명시하는 것이 좋으나, 실제로 사용할 떄는 컴파일러의 유추를 활용하는 경우가 많기 때문에,
    실제로 타입을 명시하는 경우는 잘 없다.
    그렇기 떄문에 Vec<T>를 선언하는 경우가 더 많으며,
    단축 키워드로 vec! 매크로를 사용하는 것이 좋다.
    */
    let v = vec![1, 2, 3];

    /*
    벡터의 갱신
    벡터를 만들고 요소를 추가하기 위해서는 아래와 같이
    push 메소드를 사용한다.
    */
    let mut v = Vec::new();
    v.push(4);
    v.push(5);

    /*
    struct 와 마찬가지로 벡터도 스코프 밖으로 벗어났을 때 해제됩니다.
    */
    {
        let v = vec![1, 2, 3];
        // do something...
    } // <- v 가 여기서 해제.

    /*
    위의 상황에서 벡터가 해제됨은 벡터의 모든 값 또한 함께 메모리에서 소멸 된다는 것을 의미한다.
    얼추 보아서는 그리 어려운 상황이 아닌 것처럼 보이나,
    실제 벡터 요소들에 대한 참조를 생성해서 활용할 경우 상황이 좀 복잡해진다.
    아래부터는 이런 상황에서 어떻게 대처할 수 있는지 알아보자.
    */

    /* 벡터 요소 읽기 */
    /*
    벡터의 요소를 읽는 방법은 2가지가 있다.
    1. 인덱스를 사용하여 해당 위치의 값을 읽는 방법
    2. get 메소드와 인덱스를 명시하여 해당 위치의 값을 읽는 방법
    아래 코드는 2가지 예제이다.
     */

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; //index
    let third: Option<&i32> = v.get(2); // get method

    /* 위 2가지 경우를 소개하는 이유는 인덱스를 벗어난 값을 호출했을 때 보이는 양상이 다르기 때문이다. */
    // let does_not_exist: &i32 = &v[100]; // -> 오류 발생 : panic!("index out of bounds: the len is 5 but the index is 100");
    let does_not_exist: Option<&i32> = v.get(100); // -> returns None
                                                   /* 당연히 panic! 이 발생하여 프로그램이 죽는 것보다는 none을 감지하는 코드(6장 Some(&element)/None 대처)를 작성하는 것이 훨씬 좋을 것이다. */

    /*
    유효하지 않은 참조자
    아래 코드는 동작해야 할 것으로 보이지만,
    에러가 발생한다.
    이는 v 가 불변 참조를 당하고 있는 와중에 v에 변화를 주게 되어 first의 무가결성을 해칠 수 있기 때문이다.
    아래를 항상 기억하자.

    가변 요소가 불변 참조를 당할 경우, 아무리 가변 요소일지라도, 불변 참조가 끝나지 않는 한, 변화를 줄 수 없다.
    */
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; 

    v.push(6); // ->에러 발생! : first의 무결성이 해칠 수 있다.

    /* 벡터 값들에 대한 반복 */
    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    /* 
    열거형을 사용하여 여러 타입을 지정하기 
    벡터는 같은 타입의 값들만 저장 할 수 있지만, 
    열거형을 벡터에 저장하는 형식으로 다양한 타입을 벡터에 저장하여 활용할 수 있다.
    */
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        match i {
            SpreadsheetCell::Int(i) => println!("{}", i),
            SpreadsheetCell::Float(f) => println!("{}", f),
            SpreadsheetCell::Text(s) => println!("{}", s),
        }
    }
}
