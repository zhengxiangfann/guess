use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("神秘数字: {}", secret_number);

    loop {
        println!("猜一个数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        println!("你猜的数字是: {}", &guess);

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!"); 
                break;
            },
        }
    }
}
