fn main() {
    /*
    What is 'String'?

    Rust 내부에서 문자열은 딱 한가지 타입, 'str' 만 제공한다.
    그럼 지금까지 사용한 String 타입은 뭔가?
    이는 외부 라이브러리를 통해 제공한 커질 수 있고, 소유권을 따르며, UTF-8로 인코딩된 별도의 타입이다.
    대개 스트링 타입이라고 하면 위 2개를 한꺼번에 지칭한다.

    일단 'String' 타입과 'Vector'를 활용하여 유사한 활용을 해보도록 하자.
     */

    let mut s = String::new();

    /* String 초기화 */
    //1. 리터럴 변수를 생성하고, 스트링에 할당하는 경우
    let literal = "variable.to_string";
    let s = literal.to_string();
    println!("{}", s);

    //2. 리터럴에서 바로 스트링을 생성하고 할당하는 경우
    let s = "literal.to_string".to_string();
    println!("{}", s);
    
    // 위 두가지 방법은 모두 같은 방법이다.

    //3. String::from 을 활용할 수도 있다. 
    let s = String::from("String::from");
    
    // 즉 String::from 과 to.string 은 같은 기능을 한다.

    /* String 갱신 */

    // push_str 을 통해서 뒤에 덧붙일수 있다.
    let mut s1 = String::from("Hello, ");
    let s2 = "world!";
    s1.push_str(s2);
    println!("{}", s1);
    
    // push 를 통해서 뒤에 '한 글자씩' 덧붙일수 있다.
    s1.push('!');
    println!("{}", s1);

    /* Format을 이용한 접합 */
    let mut s1 = String::from("1");
    let s2 = String::from("1");
    let s3 = s1 + &s2; // s3로 s1의 소유권이 넘어가기 떄문에 여기서 s1의 수명(사용 가능 범위)이 끝난다.
    println!("{}", s3);

    /* '+' 연산자가 사용하는 add() 함수 */
    // fn add(self, s: &str) -> String {} -> 사실 + 연산자는 왼쪽 add 함수를 실행시킨다. 이 시그니처와 맞추기 위해서 기본 왼쪽에는 기반이 되는 원본을, 이후는 참조를 두었다.
    // 문제는 여러 개의 문자를 조합해야하는 경우 고려해야하는 부분이 많아진다.
    // 따라서 이럴 때 format!을 사용한다.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // format! 을 사용하면 이런식으로 문자열을 생성할 수 있다.

    /* String Indexing */
    // String 타입은 인덱싱을 지원하지 않는다.
    // 그 이유는 String 이 Vec<u8> Wrapping 하고 있기 떄문이다. 
    // 조금 더 자세히 들어가보면,
    // utf-8로 인코딩된 문자의 바이너리 바이트 수가 어떻게 메모리에 저장이 되는지 알아야한다.
    // utf-8 문자들의 경우  8비트, 즉 3자리 정수로 인코딩 됨으로 일반적인 인덱스로는 개발자가 의도하는 자리의 문자를 가지고 올 수 없다.
    // 따라서 정확한 문자열에 해당하는 인덱싱의 경우, []안에 범위를 지정하여 가지고 와야 하는데,
    // 인덱스 범위를 벗어나거나, 대개 범위를 지정하는 경우 정확한 자릿 수를 가지고 오는데 오류를 범하기 쉽기 떄문에,
    // 다른 방법을 제공하고 있다. 

    /* chars(), bytes() */
    let s = String::from("hello");
    for i in s.chars() {
        println!("{}", i);
    }
    // 위 방법은 char 타입의 한 문자별로 iterate 하는 것이다.

    let s = String ::from("hello");
    for i in s.bytes() {
        println!("{}", i);
    }
    // 위 방법은 가공되지 않은 각 문자의 byte를 출력하는 것이다.


    /* 다른 언어에 비해 rust에서 문자를 다루는 방법이 복잡하다는 것을 알아야 한다.
    하지만 이는 비 ascii 문자를 다루는 에러를 미연에 방지하는 효과가 있기 때문에 전혀 손해가 아니라는 것 또한 알아두자.
    */
}
