use rand::{thread_rng, Rng};
use std::io::{self, Write};

fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().ok();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("read failed");
    s.trim().to_string()
}

fn pick_max() -> u32 {
    loop {
        println!("Choose difficulty:");
        println!("  1) Easy   (1..=10)");
        println!("  2) Medium (1..=100)");
        println!("  3) Hard   (1..=1_000)");
        println!("  4) Custom");

        match read_line("> ").as_str() {
            "1" => return 10,
            "2" => return 100,
            "3" => return 1_000,
            "4" => {
                let s = read_line("Enter max number (>= 2): ");
                if let Ok(n) = s.parse::<u32>() {
                    if n >= 2 { return n; }
                }
                println!("Invalid custom max. Try again.\n");
            }
            _ => println!("Please enter 1, 2, 3, or 4.\n"),
        }
    }
}

fn play_round() -> bool {
    let max = pick_max();
    let secret = thread_rng().gen_range(1..=max);
    let mut attempts = 0u32;

    println!("\nI picked a number between 1 and {max}. Can you guess it?");
    loop {
        let guess_s = read_line("Your guess: ");
        let guess = match guess_s.parse::<u32>() {
            Ok(n) if (1..=max).contains(&n) => n,
            _ => {
                println!("Enter an integer between 1 and {max}.\n");
                continue;
            }
        };

        attempts += 1;
        if guess < secret {
            println!("Too low!\n");
        } else if guess > secret {
            println!("Too high!\n");
        } else {
            println!("Correct! You got it in {attempts} attempts.\n");
            break;
        }
    }

    matches!(read_line("Play again? (y/n): ").to_lowercase().as_str(), "y" | "yes")
}

fn main() {
    println!("== Rust Number Guessing Game ==");
    while play_round() {}
    println!("Thanks for playing!");
}

