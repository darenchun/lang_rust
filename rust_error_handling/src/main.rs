use std::io;
use std::fs::File;
use std::io::ErrorKind;

/* UNRECOVERABLE errors with panic */
/*
# unwinding issue : cleansing memory is optional.
use this in 'Cargo.toml'file
 [profile.release]
 panic = 'abort'
*/

fn main() {
    println!("1. unrecoverable 2. recoverable 3. alternatives");

    loop {
        let mut guess = String::new(); // 입력한 값을 받는 변수

        io::stdin()
            .read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다."); // 터미널 입력 호출 및 오류 처리

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("입력한 값: {} 이 입력 범위를 벗어났습니다.", guess);
                break;
            }
        }; // string => u32

        println!("입력한 값: {}", guess);

        if guess == 1 {
            println!("unrecoverable_error");
            unrecoverable_error(String::from("unrecoverable_error"));
            break;
        } else if guess == 2 {
            println!("recoverable_error_enums");
            recoverable_error_enums();
            break;
        } else if guess == 3 {
            recoverable_error_alternative();
            break;
        } else {
            // just in case
            println!("프로그램을 종료합니다.");
            break;
        }
    }
}

fn unrecoverable_error(_str: String) {
    /* "panic!" macro */
    /* printing a failure message and quits the program */
    panic!("crash and burn");
    /*
    #creates error message with custom error messaging
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    error: process didn't exit successfully: `target\debug\rust_error_handling.exe` (exit code: 101)
    // and we can get additional info about the error using 'RUST_BACKTRACE= 1' in terminal
    */
}

fn recoverable_error_enums() {
    /* when creating error isn't enough and wish to do something afterwards */

    /*
    handling with enums + match
    enum Result<T,E>{
        Ok(T), // represents the type of the value that will be returned in a success case within the Ok variant
        Err(E), // represents the type of error that will be returned
    }
    */
    let _f = File::open("hello.txt");
    let _f = match _f {
        Ok(_f) => println!("file has opened"),
        // 1.permission, 2.not found / make file
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => println!("file has created : {:?}", fc.metadata()),
                Err(e) => panic!("problem creating a file : {:?}", e),
            },
            other_error => {
                panic!("problem opening a file : {:?}", other_error)
            }
        },
    };
}

fn recoverable_error_alternative() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hellow.txt").unwrap_or_else(|error| {
                panic!("Problem Creating a file : {:?}", error);
            })
        } else {
            panic!("problem opening a file : {:?}", error);
        }
    });
}
