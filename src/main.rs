use rand::Rng;
fn main() {
    println!("Guessing num!(1 - 100)");
    println!("This a guessing game written by JeongYeham in Rust programming language.");

    let secret_number = rand::rng().random_range(0..=100);
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{} is too Small!", guess),
            std::cmp::Ordering::Greater => println!("{} is too Big!", guess),
            std::cmp::Ordering::Equal => {
                println!("You win! The secret number is: {}", secret_number);
                break;
            }
        }
    }

    println!("Press Enter to exit...");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    return;
}
