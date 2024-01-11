use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {

    let secret_number  = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop{
        println!("Enter Number ");
        let mut guess = String::new();          // 1. let mut
        io::stdin().read_line(&mut guess)       // 2. reference mut parameter
            .expect("Failed to read line");     // 3. return io::Result

        let guess: u32 = match guess.trim().parse() {   // 4. type inference
            Ok(num) => num,
            Err(_) => continue,
        }; //.expect("Please type a number!");
        // Rust allow us to shadow the previous value of guess with a new one

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {       // 5. match
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("win");
                break;
            },
        }
    }
}
