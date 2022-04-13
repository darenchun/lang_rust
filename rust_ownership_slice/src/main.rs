fn main() {
    /* 
         슬라이스도 소유권을 갖지 않는 타입이다.

         슬라이스를 이용하면 컬렉션 전체가 아니라 컬렉션 내의 연속된 요소들을 참조할 수 있다.
        여기서 간단한 프로그래밍 문제를 풀어보자. 전달된 문자열에서 첫 번째 단어만 리턴하는 함수를 작성해보자.
        만일 문자열 내에 공백이 없다면 전체 문자열이 한 단어일 것이므로, 이 때는 전체 문자열을 리턴하면 된다.
     */

    let mut s = String::from("as there is");

    let _first_word = first_word(&s); // 번수 first_word_endpoint_index 에는 2라는 값이 들어간다.
    s.clear(); // 여기서 s 값을 공백으로 만들고 있지만, 이미 변수 first_word_endpoint_index 에 값이 할당되고 난 이후에 발생하는 행위이므로 여전히 위 변수에는 2라는 값이 들어있다.

    /* 
         문제는 clear를 하고 난 다음에 2라는 값은 사실이 아니게 된다는 점이다. 즉 원본이 변화한 상태에서 찾은 첫 단어의 길이 값은 사실이 아니게 된다.
         이런 상황에서 이 번에는 두번 째 단어를 찾아야 한다고 생각해보자. 첫 단어의 끝 부터 해당 단어의 끝이므로 2개의 길이를 추적해야 한다. 
    */

    /* rust 에서는 이와 같은 문제를 slice로 해결하고 있다. */

    let mut _s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];

    /*  s 라는 문자열에 해당하는 참조를 얻어오는 방법은 유사하지만, '[]' 를 통해 문자열 중 지정된 부분에 대한 참조만을 하게 되었다.*/
    /*  대괄호 안에
        [starting_idx..end_idx]
        형식으로 시작 위치로부터 슬라이스의 길이(end - start)만큼의 데이터를 저장하는 구조체로 활용할 수 있다.
        
        이를 토대로 first_word 함수를 다시 쓰면...(하단 참고)
        이전에 내재된 버그는 이미 첫 번째 단어의 인덱스를 찾고 난 이후 단어가 바뀌고 나면 그 변동을 감안할 수 없다는 점에서 버그를 가지고 있었다.
        하지만 하단 코드를 실행하면 이제 원본 값에 변화를 주는 's.clear();' 부분에서 아래와 같은 에러가 발생한다.
       
        println!("the first word is : {}", first_word);
    
        [에러 :  "cannot borrow `s` as mutable because it is also borrowed as immutable mutable borrow occurs here"]

        앞서 rust가 가지고 있는 규칙을 상기해보게 되면
        만일 불변 참조를 이미 생성한 경우라면 원본에 변화를 줄 수 없다는 것을 기억할 것이다.
        하지만 위 코드는 원본 String 타입의 값을 제거해야 하므로 clear()를 실행하기 위해선 가변 참조를 시도하고 이는 rust 규칙에 어긋나므로 컴파일에 실패한다.
    */


    /* 문자열 리터럴(실제 값)은 Slice다! */
    /*  앞서 바이너리에 저장되는 문자열 리터럴에 대해 설명했던 것을 기억할 것이다. 이제는 슬라이스에 대해 이해했으므로 문자열 리터럴도 완벽하게 이해할 수 있다.*/

    // let s = "Hello world";

    // 위 코드에서 s 의 타입은 &str이다. 즉 바이너리의 어느 한 지점을 가리키는 슬라이스라는 의미다. 그렇기 때문에 문자열 리터럴은 항상 불변이다.
    // 가리키는 문자열이 변동하지 않는다는 전제하에 어느 한 지점을 가르키는 것이기 때문이다.

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

