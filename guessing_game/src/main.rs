use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("숫자를 맞춰봅시다.");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("guess answer : {}", secret_number);
    
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("입력한 값을 읽지 못했습니다.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("입력한 값: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Lower!!"),
            Ordering::Greater => println!("Greater!!"),
            Ordering::Equal => {
                println!("Equal Correct!!");
                break;
            }
        }
    }
}

