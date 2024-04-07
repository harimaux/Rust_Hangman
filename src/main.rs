use rand::seq::IteratorRandom;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

//GET RANDOM FRUIT FROM VETROR AND REMOVE THE ONE THAT WAS PICKED
fn pick_random_fruit(fruits: &mut Vec<String>) -> Option<String> {
    let mut rng = rand::thread_rng();
    if let Some(index) = (0..fruits.len()).choose(&mut rng) {
        Some(fruits.remove(index))
    } else {
        None
    }
}

fn main() {
    //OPEN FILE AND GET LIST OF ALL FRUITS
    let file = match File::open("fruits.txt") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening fruits file: {}", err);
            return;
        }
    };

    let reader = io::BufReader::new(file);

    //REMOVES DUPLICATES FROM THE LIST
    let mut unique_fruits = HashSet::new();

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                unique_fruits.insert(line.trim().to_string());
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    //FRUITS VECTOR
    let mut fruits: Vec<String> = unique_fruits.into_iter().collect();
    println!("Welcome to Hangman!");

    //MAIN GAME LOOP
    loop {

        match pick_random_fruit(&mut fruits) {
            Some(random_fruit) => {
                let mut hidden_word = String::new();
                let revealed_word = random_fruit.clone();

                //REPLACE THE LETTERS WITH UNDERSCORES
                for c in revealed_word.chars() {
                    if c == ' ' {
                        hidden_word.push(' ');
                    } else {
                        hidden_word.push('_');
                    }
                }

                //println!("Guess the word: {}", hidden_word);
                let mut attempts = 36;

                //GUESSING LOOP
                loop {

                    //GETS USER INPUT
                    println!("Enter a letter or guess the entire word: {}",hidden_word);
                    let mut guess = String::new().to_lowercase();
                    io::stdin().read_line(&mut guess).expect("Failed to read line");

                    //CHECK IF PLAYER GUESSED THE ENTIRE WORD
                    if guess.trim().to_lowercase() == revealed_word.to_lowercase() {
                        println!("Congratulations! You guessed the word: {}", revealed_word);
                        break;
                    }

                    //SINGLE CHARACTER INPUT CHECK
                    if guess.trim().chars().count() == 1 {
                        //CHECK IF THE GUESSED LETTER IS IN THE HIDDEN WORD
                        if revealed_word.to_lowercase().contains(&guess.trim().to_lowercase()) {
                            //UPDATE THE UNDERSCORES IN HIDDEN WORD
                            for (i, c) in revealed_word.chars().enumerate() {
                                if c.to_lowercase().to_string() == guess.trim().to_lowercase() {
                                    hidden_word.replace_range(i..=i, &c.to_string());
                                }
                            }
                            println!("Correct guess! Current word: {}", hidden_word);

                            //CHECK IF THE ENTIRE WORD HAS BEEN REVEALED
                            if hidden_word == revealed_word {
                                println!("Congratulations! You guessed the word: {}", revealed_word);
                                break;
                            }

                        } else {
                            //INCORRECT LETTER GUESS
                            println!("Incorrect guess!");
                            attempts -= 1;
                            println!("Attempts left: {}", attempts);
                            if attempts == 0 {
                                println!("Game over! The word was: {}", revealed_word);
                                break;
                            }
                        }
                    } else {
                        //INCORRECT WORD GUESS
                        println!("Incorrect guess!");
                        attempts -= 1;
                        println!("Attempts left: {}", attempts);
                        if attempts == 0 {
                            println!("Game over! The word was: {}", revealed_word);
                            break;
                        }
                    }
                }
            }
            None => {

                break;
            }
        }

        //FINISHES THE GAME IF ALL WORDS FROM THE FILE BEEN USED
        if fruits.len() == 0 {
            println!("You have finished the game, well done you, now go play outside!");
            break;
        }

        //OFFERS TO PLAY AGAIN OF FINISH THE GAME
        let mut play_again = String::new();
        loop {
            play_again.clear();
            println!("Do you want to play again? (yes/no)");
            io::stdin().read_line(&mut play_again).expect("Failed to read line");
            let play_again = play_again.trim().to_lowercase();
            if play_again == "yes" {
                break;
            } else if play_again == "no" {
                println!("Ok, thanks for playing!");
                return;
            } else {
                println!("Invalid input! Please enter 'yes' or 'no'.");
            }
        }

        
    }
}