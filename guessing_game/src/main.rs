use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {

    let secret_number  = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop{
        println!("Enter Number ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; //.expect("Please type a number!");

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("win");
                break;
            },
        }
    }
}
