use std::cmp::Ordering;
use std::io;
use std::io::Write;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
	print!("Please input your guess: ");
	io::stdout().flush().expect("not flushed i guess");
	
	let mut guess = String::new();
	
	io::stdin().read_line(&mut guess).expect("Failed to read line");

	let guess: u32 = guess.trim().parse().expect("Please type a number!");

	match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
		println!("YAY YAY YAY YOU FOUNDED IT");
		break;
	    }
	}
    }
}
