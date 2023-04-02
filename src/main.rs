use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the numbew, bitch 🤔🤔!");
    let secret_number = rand::thread_rng().gen_range(69..=420);
    // println!("The secwet numbew iws uwu:{secret_number}, Bitch");

    loop {
        println!("Pwease input youw guess, Bitch");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Faiwed tuwu wead wine, Bitch~😒😒😒😒😒");

        let guess: u32 = match guess.
            trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue, 
            };        
            println!("Uwu guessed: {guess}, Bitch😱😱");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("too smaww 😭😭!"),
                Ordering::Greater => println!("too wawge 👀👀!"),
                Ordering::Equal => {
                    println!("congwatuwations bitch! 😍😍😍👌👌👌👌👌\nduwu uwu wawnt a pwize??🙀🙀🙀");
                    println!("uwu win !!!!!!");
                    break;
                }
            }
    };
    
    }