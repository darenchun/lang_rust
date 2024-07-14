/*
제네릭(범용)의 활용

제네릭은 언제 쓸 까?
특정 기능의 과정이 같거나 비슷하지만, 여러가지 타입을 혼재 해서 리턴하거나 인자로 활용하기 위해서 만들어졌다.
*/

// 가장 큰 정수
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

// 가장 큰 글자(8비트 변환 시)
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return largest;
}

// 위 두 함수는 모두 "가장 큰 어떤 것을 찾는다"는 기능이 동일하지만, 돌려주는 타입만 다른 케이스다.
// 2가지 함수로 기능이 분리되어 있지만, 이를 하나의 "범용/포괄(generic)"적인 함수 하나로 압축할 수 있다.
// 여기서 'T' 는 '아직 정해지지 않았지만 실행시 정해지게 될 어떤' 타입을 나타낸다. 미정의 어떤 타입으로 이해하면 되겠다.
/* fn largest_<T>(list: &[T])-> T {
    let mut largest = list[0];

    // an implementation of `std::cmp::PartialOrd` might be missing for `T`
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    return largest[0];
} */

/* 구조체 필드 또한 범용으로 남겨둘 수 있다.
구조체 명 내에 '범용: T' 변수가 있음을 '<T>'을 통해 알려주면 된다.
다만 여기 사용될 수 있는 범용 변수의 타입은 동일, 즉 한 가지 타입만 허용된다는 것을 의미한다.
*/
struct Point<T> {
    x: T,
    y: T,
}

/*
구조체 내부에 아직 정의되지 않았지만 복수의 타입이 포함 될 수 있다면 어떻게 표현할까?
간단하게 하나의 범용 변수를 더 적어주면 된다.
*/
struct Point_with_multiple_types<T, U> {
    x: T,
    y: U,
}

struct Tuple<T> {
    first: T,
    second: T,
}

use std::ops::{Add};

use crate::traits::Max; // 더할 수 있다는 trait/ 특징을 적용시키기 위해 표준 라이브러리를 가져온다.

// Add 타입의 특징을 가지고 있다는 것을 명시해준다.
fn sum<T: Add<Output = T>>(tuple: Tuple<T>) -> T {
    // return tuple.first + tuple.second; // 에러가 발생한다 : 'T' 해당 타입이 '+' 연산을 할 수 있는지 없는지 알 수가 없기 떄문이다.
    return tuple.first + tuple.second;
}

pub mod extend_types;
pub mod traits;
pub mod derive;

fn main() {
    let numbers = vec![90, 1, 2, 66, 7, 99];
    let result = largest_i32(&numbers);
    println!("the largest number is {}", result);

    let chars = vec!['t', 'y', 'z', 'z', 'a'];
    let result = largest_char(&chars);
    println!("the largest char is {}", result);

    let _integer = Point { x: 5, y: 5 };
    let _float = Point { x: 1.0, y: 2.0 };
    let integer_plus_float = Point_with_multiple_types { x: 5, y: 1.0 };
    println!(
        "integer_plus_float variants {},{}",
        integer_plus_float.x, integer_plus_float.y
    );

    let tuple = Tuple {
        first: 1,
        second: 4,
    };
    println!("sum of variants in tuple {}", sum(tuple));

    // making same implementation for different use cases
    let two_tuple: traits::TwoTuple<u32> = traits::TwoTuple {
        first: 1u32,
        second: 4u32,
    };
    let three_tuple: traits::ThreeTuple<u32> = traits::ThreeTuple {
        first: 2u32,
        second: 3u32,
        third: 5u32,
    };

    println!("{}", two_tuple.max());
    println!("{}", three_tuple.max());

    trait Sawtooth {
        fn sawtooth(&self) -> Self;
    }

    // extending the trait for builtin f6 type
    impl Sawtooth for f64 {
        fn sawtooth(&self) -> f64 {
            return self - self.floor();
        }
    }

    // making additional functionalities for existing types
    println!("{}", 2.234343434f64.sawtooth());


    let p: derive::Point<u32> = derive::Point { x: 4u32, y: 2u32 };

    //using display
    println!("{}",p);
    println!("{:?}",p);

    println!("{}",generic_type_return(10));
    println!("{}",generic_type_return("something"));
    println!("{}",generic_type_return('C'));

}

fn generic_type_return<Generic> (input : Generic) -> Generic {
    return input;
}
