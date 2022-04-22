/*
    구조체(struct/structure)는 서로 관련이 있는 여러 값을 의미 있는 하나로 모으고, 이름을 지정해 접근할 수 있는 사용자 정의 데이터 타입이다.
    객체지향언어에 익숙하다면, 구조체는 객체의 데이터 속성과 같다고 생각해도 무방하다.
    해당 패키지에서는 구조체의 특징에 대해서 정리해보도록 한다.
    [주요 관점]
    1. 튜플과의 차이점
    2. methods, associated functions
*/

fn main() {
    // 구조체의 정의를 위해서는 'struct' 키워드 다음에 구조체에 부여할 이름을 지정해주면 된다. 그 이름은 해당 구조체 내부에 들어갈 데이터를 대표할 수 있는 이름이면 좋다. 구조체는 대개 대문자로 시작하는 것을 관습화한다.
    struct User {
        user_name: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 구조체를 정의한 후 이를 사용하려면, 각 필드에 저장할 값을 명시해서 구조체의 인스턴스(해당 구조체의 실제 구현체)를 생성해야 한다. 예시는 아래와 같다. json 과 같은 형식에서 자주 봤겠지만, "key:value," 형식을 취한다.
    let mut instance_of_user = User {
        user_name: String::from("someusername123"),
        email: String::from("someuseremail123@example.com"),
        active: true,
        sign_in_count: 123, // 꼭 구조체의 선언 순서와 같을 필요는 없다.
    };

    /*
    [※주의]: 구조체의 인스턴스 자체는 가변이다.
    다른 객체지향 언어와는 다르게 rust는 특정 필드만을 가변으로 표시하는 것을 지원하지 않는다.
    */


    // 구조체에서 원하는 값을 읽으려면 '.'를 이용하면 된다. 만약 해당 변수가 가변 변수라면 새로운 값을 할당해 기존 값을 대체할 수도 있다.
    instance_of_user.user_name = String::from("새로운_이름");
    println!("user_name of instance : {}", instance_of_user.user_name);

    // 다른 언어와 같이 '생성자:builder'와 같은 역할을 하는 함수를 만들어 구조체 인스턴스 생성을 원활하게 할 수 있다. 묵시적으로 해당 인스턴스를 리턴하게 해두어 변수에 바로 할당 할 수 있게 한다.
    fn build_user (user_name:String, email:String) -> User {
        return User { user_name: user_name, email: email, sign_in_count: 1, active: true };
    }

    // 대개 습관상, 파라미터로 보내는 인수의 변수명과, 필드명이 같게 만들어 가독성을 높이는 경우가 많기 때문에, 파라미터와 구조체의 필드명이 같을 경우 한번만 적어도 자동으로 할당을 할 수 있는 문법을 제공한다.
    fn build_user_2 (user_name:String, email:String) -> User {
        return User { user_name, email, sign_in_count: 1, active: true }; // 위 생성자와 같은 문법이 된다.
    }

    /* 기존의 인스턴스로부터 새 인스턴스 생성하기 */
    // 기 존재하는 인스턴스에서 몇 가지 필드의 값만 수저안 상태의 새 구조체 인스턴스가 필요한 경우 '구조체 갱신 문법'을 이용하면 편리하다.

    let mut user2 = User {
        email: String::from("email2"),
        user_name : String::from("user2_name"),
        active : instance_of_user.active, // 문법 상 기존에 생성된 인스턴스들이 있다면, 해당 인스턴스들의 필드 값을 읽어 바로 할당을 할 수 있다.
        sign_in_count: instance_of_user.sign_in_count,
    };

    // 아래는 아예 특정 인스턴스와 값의 차이가 없을 경우 활용할 수 있는 방법이다.
    let mut user3 : User = User {
        email : String::from("email3"),
        user_name : String::from("user2_name"),
        ..instance_of_user // 나머지는 최초 인스턴스인 'instance_of_user'과 내용을 같이 하겠다는 의미가 된다.
    };

    /*
    튜플 구조체 : 필드의 이름이 없고 값만 존재하는 tuple과 유사한 형태의 구조체도 생성이 가능하다.
    */
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    /*
    구조체의 소유권 : 구조체가 특정 변수나 다른 구조체의 값을 참조하여 그 값을 저장할 수 는 있으나, 
    궁극적인 원본의 값이 의도적이지 않게 바뀌게 될 경우, 지금 사용하는 구조체의 필드 값이 의도적이지 않게 변하게 된다.
    이러한 '참조 값을 할당'하는 경우의 관리를 'lifetime'이라고 하는데 차후 배우게 될 것이다. 
    */


}
