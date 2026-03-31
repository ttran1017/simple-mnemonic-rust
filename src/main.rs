mod words;

use std::io;

fn main() {
    let mut score: u32 = 0;
    
    loop {
        let mut guess: String = String::new();
        let random_number = rand::random_range(0..=99);
        let target = words::WORD_TABLE[random_number];

        header(score);
        println!("Enter word for {random_number} | target = {target}");
        
        loop {
            match io::stdin()
                .read_line(& mut guess) {
                    Ok(guess) => guess,
                    Err(_) => continue,
                };
            break;
        }

        clear_screen();
        let guess = guess.trim();
        println!("Guess was: {guess} | Target was: {target}");
        if guess == target {
            score += 1;
        }
    }
}

fn header(score: u32) {
    println!("\nMNEMONIC TRAINER | SCORE = {score}");
    println!("==============================");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    print!("\x1B[3J");
}
