use std::io;
fn main() {
    /*
        write # for a sample code result.
        1: if
        2: for
        3: while
        4: loop
    */
    loop {
        let mut user_input: String = String::new();
        println!("숫자를 입력 하세요. 4 입력시 프로그램 종료");
        io::stdin()
            .read_line(&mut user_input)
            .expect("입력한 값을 읽지 못했습니다.");

        // 문자 가공 후 숫자로 변환하고 숫자가 아닌경우 에러 송출
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if user_input == 1 {
            _if(user_input);
        } else if user_input == 2 {
            _for(user_input);
        } else if user_input == 3 {
            _while(user_input)
        } else if user_input == 4 {
            break;
        }
    }
}

fn _if(_x: u32) -> bool {
    let _if_x: bool = if _x == 1 { true } else { false };
    println!("example of if");
    println!("current _if_x = {}", _if_x);
    return _if_x;
}

fn _for(_x: u32) {
    let _for_x: bool = if _x == 2 { true } else { false };

    // int 값 치환후 해당 값 만큼 반복 할 수 있도록 처리
    loop {
        let mut user_input: String = String::new();
        println!("반복 하고 싶은 숫자를 입력 하세요. % exit 입력 시 해당 항목 이탈 %");

        // "입력 값이 exit 일 경우 loop 탈출 후 바로 return"

        io::stdin()
            .read_line(&mut user_input)
            .expect("입력한 값을 읽지 못했습니다.");

        // exit 이란 값을 입력 했을 경우 loop 탈출
        if user_input.trim() == "exit" {
            break;
        }

        // 문자 가공 후 숫자로 변환하고 숫자가 아닌경우 에러 송출
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_err) => continue,
        };

        println!("{}만큼 숫자 반복 출력", user_input);

        for i in 1..=user_input {
            println!("{}", i)
        }
    }
}

fn _while(_x: u32) {
    let _for_x: bool = if _x == 3 { true } else { false };

    // int 값 치환후 해당 값 만큼 반복 할 수 있도록 처리
    loop {
        let mut user_input: String = String::new();
        println!("반복 하고 싶은 숫자를 입력 하세요. % exit 입력 시 해당 항목 이탈 %");

        // "입력 값이 exit 일 경우 loop 탈출 후 바로 return"

        io::stdin()
            .read_line(&mut user_input)
            .expect("입력한 값을 읽지 못했습니다.");

        // exit 이란 값을 입력 했을 경우 loop 탈출
        if user_input.trim() == "exit" {
            break;
        }

        // 문자 가공 후 숫자로 변환하고 숫자가 아닌경우 에러 송출
        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_err) => continue,
        };

        println!("{}만큼 숫자 반복 출력", user_input);

        let mut i = 1;
        while i <= user_input{
            println!("{}", i);
            i = i + 1;
        }
    }
}
