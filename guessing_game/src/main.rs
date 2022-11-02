use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input tour guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //也可以这样写
        //io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        //trim 方法会消除 \n 或者 \r\n，只留下 5。
        println!("You guessed : {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater=> println!("Too big!"),
            Ordering::Equal=> {
                println!("You win");
                println!("The secret number is: {secret_number}");
                break
            }
        }
    }
}
