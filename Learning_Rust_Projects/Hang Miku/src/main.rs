use std::io::{self, Write};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::{thread_rng, seq::IteratorRandom};

fn print_banner() {
    let banner = r#"
 , ,  _   ,  ,  _,     , , ___, ,  ,,  , 
 |_|,'|\  |\ | / _    |\/|' |   |_/ |  | 
'| |  |-\ |'\|'\_|`   | `| _|_,'| \'\__| 
 ' `  '  `'  `  _|    '  `'     '  `   ` 
            '                        
Save Miku from the hangman's noose!
Use your wits and guess the letters to keep
Miku safe and reveal the hidden word.

Can you prevent her from being hanged?
Test your skills and find out!
"#;
    println!("{}", banner);
}

fn print_hangman(attempts: usize) {
    let hangman = [
        "  +----+\n   |   |\n       |\n       |\n       |\n       |\n=========\n",
        "  +----+\n   |   |\n (T.T) |\n       |\n       |\n       |\n=========\n",
        "  +----+\n   |   |\n (T.T) |\n   |   |\n       |\n       |\n=========\n",
        "  +----+\n   |   |\n (T.T) |\n  /|   |\n       |\n       |\n=========\n",
        "  +----+\n   |   |\n (T.T) |\n  /|\\  |\n       |\n       |\n=========\n",
        "  +----+\n   |   |\n (T.T) |\n  /|\\  |\n  /    |\n       |\n=========\n",
        "  +----+\n   |   |\n (T.T) |\n  /|\\  |\n  / \\  |\n       |\n=========\n",
    ];
    println!("{}", hangman[6 - attempts]);
}

fn get_random_line() -> Result<String, io::Error> {
    let file = File::open("../words.txt")?;
    let file = BufReader::new(file);
    let mut rng = thread_rng();
    let sample = file.lines().choose(&mut rng).unwrap()?;
    Ok(sample.to_lowercase())
}

fn main() {
    print_banner();

    let word = get_random_line().expect("Failed to get a random word"); // Get the word from the file
    let word: Vec<char> = word.chars().collect();
    let mut guessed_word: Vec<char> = vec!['_'; word.len()];
    let mut attempts = 6;
    let mut guessed_letters: HashSet<char> = HashSet::new();

    while attempts > 0 && guessed_word.contains(&'_') {
        print_hangman(attempts);
        println!("Word: {}", guessed_word.iter().collect::<String>());
        println!("Guessed letters: {:?}", guessed_letters);
        println!("Attempts remaining: {}", attempts);

        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: char = match guess.trim().chars().next() {
            Some(c) => c,
            None => continue,
        };

        if guessed_letters.contains(&guess) {
            println!("You already guessed that letter.");
            continue;
        }

        guessed_letters.insert(guess);

        if word.contains(&guess) {
            for (i, &c) in word.iter().enumerate() {
                if c == guess {
                    guessed_word[i] = guess;
                }
            }
        } else {
            attempts -= 1;
        }
    }

    print_hangman(attempts);
    if attempts > 0 {
        println!("Congratulations! You've guessed the word and saved Miku!: {}", word.iter().collect::<String>());
    } else {
        println!("Miku has been hanged! The word was: {}", word.iter().collect::<String>());
    }
}
