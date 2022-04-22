
fn main() {
    /* 예시로 사각형의 넓이를 구하는 프로그램을 짜보도록 한다. */
    let length_1: u32 = 100;
    let width_1: u32 = 100;

    println!(
        "The area of the rectangle is {} square pixels",
        area(length_1, width_1)
    );

    /* 변수 기준으로 생성을 하는 방법도 있지만 "튜플"을 활용해서 하나의 인스턴스로 계산을 끝낼 수도 있다. */
    let rec_1 = (50, 100);
    
    println!(
        "The `area of the rectangle is {} square pixels",
        area2(rec_1)
    );
    
    /* 위 튜플을 활용한 방법도 좋지만, 인자의 내용을 표현하기는 조금 불충분하다. 이번에는 struct 를 활용해보자. */
    
    let rec_2 = Rectangle {length : 32, width : 30};
    println!(
        "The `area of the rectangle is {} square pixels",
        area3(&rec_2)

    );
    /* 파생 트레잇(derived trait)으로 유용한 기능 추가하기 */
    
    
}


/* 첫 예시 사용 함수*/
fn area(length_1: u32, width_1: u32) -> u32 {
    return width_1 * length_1;
}

/* 두 번째 예시 함수 : paramemter as tuple*/
fn area2(dimensions : (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

/* 세 번째 예시 struct 및 사용 함수 : paramemter as struct*/
struct Rectangle {
    length : u32,
    width : u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    return rectangle.length * rectangle.width;
}
