use std::collections::HashMap;

fn main() {
    /*
    HashMap<K,V> 타입은 K(ey) 타입의 키에 V(alue) 타입의 값을 매핑한 것을 저장한다.
    이 매핑은 해쉬 함수를 통해 동작하는데, 해쉬 함수는 이 키와 값을 메모리 어디에 저장할 지 정해준다.
    해당 타입은 인덱스가 아닌 키값을 통해 값을 찾을 때 유용하게 활용가능하다.
    이번 프로젝트에서는 가장 기본적인 함수들에 대해서 배워 볼 것이다.
     */

     // HashMap 생성
    let mut scores = HashMap::new();
    // insert 를 통한 값 할당
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 개별적 벡터 값을 생성한 후 각 인덱스 별로 키, 값을 짝지어 hashmap 생성
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // 어떤 타입의 키와 값이 들어갈 지 모르기 때문에 '_'를 사용한다.
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    /* 해쉬맵과 소유권 */
    let field_name = String::from("Blue");
    let field_value = String::from("Favorite Color");

    let mut map = HashMap::new();

    map.insert(field_name,field_value); // 소유권이 원래 변수에서 인자로 넘어가기 때문에 원본 변수는 사용할 수 없다.

    /* 해쉬맵 인자 접근 */
    // get 메소드를 통해 해쉬맵에서 키에 해당하는 값을 얻을 수 있다.
    let mut key: String = String::from("Blue");
    let value = map.get(&key);
    println!("{:?}", value);

    // for loop를 통해 각 키, 값을 다 구할 수도 있다.
    for (k,v) in &map {
        println!("[key : value] {} : {}", k, v);
    }

    /* 해쉬맵 갱신하기 */
    
    // 특정 키에 해당하는 값을 덮어쓰기 
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("initial {:?}", scores);
    scores.insert(String::from("Blue"), 25);
    println!("after {:?}", scores);

    // 키에 할당된 값이 없는 경우에만 값을 할당하기
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    //entry(key).or_insert(value) -> 특정 키가 있는지, 있을 경우 값이 있는지 검증(entry)하고 없을 경우 value를 추가한다(or_insert).
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);
    println!("{:?}", scores);

    // 예전 값을 기초로 값을 갱신하기
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let mut count = map.entry(word).or_insert(0); //&참조를 걸고 있는 값인데 수정이 되는 값
        *count += 1;
    }
    println!("{:?}", map);

    

}
