use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
#[allow(unused)]


fn hangman(word: String) -> i32 {
    let mut word_list: Vec<String> = get_words_from_file(word.len());
    let mut penalties: i32 = 0;
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut obfuscated_word = obfuscate_word(&word, &guessed_letters);

    while penalties < 1000 {
        obfuscated_word = obfuscate_word(&word, &guessed_letters);
        word_list = filter_words(&word_list, &obfuscated_word, &guessed_letters);
        let guess = guess_ai(&word_list, &guessed_letters, &obfuscated_word).unwrap();
        let penalty = handle_guess(&guess, &mut guessed_letters, &word);
        if penalty == -1 {
            break;
        }
        penalties += penalty;
    }
    penalties
}

fn format_words(words: Vec<String>) -> Vec<String> {
    let mut formatted =
        words
            .iter()
            .map(|s| s.to_lowercase())
            .filter(|s| s.chars().all(|c| {c.is_ascii_alphabetic() && c.is_ascii_lowercase()}))
            .collect::<Vec<String>>();
    return formatted;
}
fn get_words_from_file(length : usize) -> Vec<String> {
    let mut f = File::open("src/words_alpha.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let words= contents.lines().map(|s| s.to_string()).filter(|s| s.len() == length).collect();
    words
}

fn most_frequent_letter(words: &Vec<String>, guessed_letters: &Vec<char>) -> Option<char>{
    if words.len() <= 1 {
        return None;
    }

    let mut letter_counts: HashMap<char, i32> = HashMap::new();

    for word in words{
        for letter in word.chars(){
            if guessed_letters.contains(&letter){
                continue;
            }
            letter_counts.entry(letter).and_modify(|c| {*c += 1}).or_insert(0);
        }
    }

    Some(*letter_counts.iter().max_by_key(|&(_, count)| count)?.0)
}

fn guess_ai(words: &Vec<String>, guessed_letters: &Vec<char>, obfuscated_word: &String) -> Result<String, String> {
    if words.len() < 1 {
        return Err("no words to guess from".to_string());
    }

    if !obfuscated_word.contains('_'){
        return Ok(obfuscated_word.clone());
    }
    
    let letter = most_frequent_letter(words, guessed_letters);
    match letter{
        Some(l) => Ok(l.to_string()),
        None => Ok(words.first().unwrap().to_string()),
    }
}

fn handle_guess(input : &String, guessed_letters: &mut Vec<char>, word: &String) -> i32 {

    // guard clause to check if input is valid
    if !input.chars().all(|c| {c.is_ascii_alphabetic() && c.is_ascii_lowercase()}) || input.is_empty(){
        dbg!(input);
        println!("Invalid input");
        return 0;
    }

    match input.len(){
        // guess is a letter
        1 => {
            let c = input.chars().next().unwrap();
            if guessed_letters.contains(&c){
                0
            }

            else {
                guessed_letters.push(c);
                1
            }

        },

        // guess is a word
        _ => {
            if word == input {
                -1
            }
            else {
                1
            }
        }
    }
}

fn filter_words(words: &Vec<String>, obfuscated_word: &String, guessed_letters: &Vec<char>) -> Vec<String> {
    let mut new_words = vec![];
    // remove all the words that do not match the obfuscated word
    for word in words.iter(){
        let mut valid = true;
        for i  in 0..word.len(){
            if  (obfuscated_word.chars().nth(i).unwrap() != word.chars().nth(i).unwrap()
                && obfuscated_word.chars().nth(i).unwrap() != '_')
                || (!obfuscated_word.contains(word.chars().nth(i).unwrap())
                && guessed_letters.contains(&word.chars().nth(i).unwrap()))
            {
                valid = false;
                break;
            }
        }
        if valid {
            new_words.push(word.clone());
        }
    }
    new_words
}


fn obfuscate_word(word: &String, guessed_letters: &Vec<char>) -> String {
    word.chars().map(|c| {
        if guessed_letters.contains(&c){
            c
        }
        else {
            '_'
        }
    }).collect()
}

fn print_hangman(penalties: i32){
    let hangman = vec![
        "  +---+\n  |   |\n      |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n      |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n  |   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|   |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n      |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n /    |\n      |\n=========",
        "  +---+\n  |   |\n  O   |\n /|\\  |\n / \\  |\n      |\n========="
    ];

    println!("{}", hangman[penalties as usize]);
}


fn main() {
    let start = std::time::Instant::now();
    let mut f = File::open("src/words_alpha.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    let raw = contents.lines().map(|s| s.to_string()).filter(|x| { x.len() >= 2 }).collect::<Vec<String>>();

    let words = format_words(raw);


    let mut res = 0;
    let size = words.len();
    println!("Size: {}", size);
    let mut counter = 0;
    for word in words{

        println!("Word: {}, Counter: {}", word, counter);
        counter += 1;
        res += hangman(word);
    }
    println!("Total penalties: {}", res);
    println!("Average penalties: {}", res / size as i32);

    println!("Time: {:?}", start.elapsed());
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_hangman(){
        hangman("that".to_string());
    }
    #[test]
    fn test_print_hangman() {
        print_hangman(6);
    }


    #[test]
    fn test_get_words_from_file(){
        let words = get_words_from_file(4);
        assert_eq!(words.first().unwrap(), "allo");
    }

    #[test]
    fn test_most_frequent_letter(){
        let words = vec!["test".to_string(), "best".to_string()];
        let guessed_letters = vec!['t', 'e', 's'];
        let letter = most_frequent_letter(&words, &guessed_letters);
        assert_eq!(letter,Some('b'));
    }

    #[test]
    fn test_most_frequent_letter_no_words(){
        let words = vec![];
        let guessed_letters = vec!['t', 'e', 's'];
        let letter = most_frequent_letter(&words, &guessed_letters);
        assert_eq!(letter,None);
    }

    #[test]
    fn test_guess_ai_no_words(){
        let words = vec![];
        let guessed_letters = vec!['t', 'e', 's'];
        let guess = guess_ai(&words, &guessed_letters, &"____".to_string());
        assert_eq!(guess, Err("no words to guess from".to_string()));
    }

    #[test]
    fn test_guess_ai(){
        let words = vec!["test".to_string(), "best".to_string()];
        let guessed_letters = vec!['t', 'e', 's'];
        let guess = guess_ai(&words, &guessed_letters, &"____".to_string());
        assert_eq!(guess, Ok("b".to_string()));
    }
    #[test]
    fn test_guess_ai_one_word(){
        let words = vec!["test".to_string()];
        let guessed_letters = vec![];
        let guess = guess_ai(&words, &guessed_letters, &"____".to_string());
        assert_eq!(guess, Ok("test".to_string()));
    }

    #[test]
    fn test_handle_guess(){
        let mut guessed_letters = vec![];
        let word = "test".to_string();
        let penalty = handle_guess(&"t".to_string(), &mut guessed_letters, &word);
        assert_eq!(penalty, 1);
    }

    #[test]
    fn test_handle_guess2(){
        let mut guessed_letters = vec![];
        let word = "test".to_string();
        let penalty = handle_guess(&"".to_string(), &mut guessed_letters, &word);
        assert_eq!(penalty, 0);
    }

    #[test]
    fn test_handle_guess3(){
        let mut guessed_letters = vec![];
        let word = "test".to_string();
        let penalty = handle_guess(&"test".to_string(), &mut guessed_letters, &word);
        assert_eq!(penalty, -1);
    }
}