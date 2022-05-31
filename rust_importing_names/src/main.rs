// 'pub' is a visibility modifier, which means that the function is public.
// 'mod' is a module modifier, which means that the function is a module.

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested modules");
            }
        }
    }
}

pub mod b {
    pub fn some_function() {
        println!("called some function");
    }
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}



// 자주 사용할 경우에는 이렇게 사용하는 것이 불편하고 가독성도 좋아진다. 일종의 alias 및 단축키를 사용하는 것이다.
use a::series::of::nested_modules as a;
use b::some_function as b;
use TrafficLight::{Red, Green};

fn main() {
    // the function in mod can be used with '::', while functions in structs can be used with '.'

    /*
        It is not entirely clear to say but this will give a gist about the relation between mod and struct and library.
        "lib(mod(struct(traits)functions))""
    */
    a();
    b();
    println!("Red: {:?}", Red);
    println!("Green: {:?}", Green);
    println!("Yellow: {:?}", TrafficLight::Yellow); // 해당 부분은 use 에 명시하지 않았으므로 모두 enum 명까지 모두 적어준다.

}
