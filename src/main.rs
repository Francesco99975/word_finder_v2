use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use colored::Colorize;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn get_combinations(word: &str) -> Vec<String> {
    let chars: Vec<char> = word.chars().collect();
    let max = word.len();
    let mut combinations: Vec<String> = Vec::new();

    for length in (2..=max).rev() {
        generate_combinations(&chars, &mut combinations, String::new(), 0, length);
    }

    combinations
}

fn generate_combinations(chars: &[char], result: &mut Vec<String>, current: String, index: usize, rem_len: usize) {
    if rem_len == 0 {
        return;
    }

    for i in index..chars.len() {
        let mut next_combination = current.clone();
        next_combination.push(chars[i]);
        if !result.contains(&next_combination) {
            result.push(next_combination.clone());
        }
        generate_combinations(chars, result, next_combination, i + 1, rem_len - 1);
    }
}

fn get_permutations(word: &str) -> Vec<String> {
    let mut permutations: Vec<String> = Vec::new();
    let mut characters_list: Vec<char> = word.chars().collect();

    for permutation in permutohedron::Heap::new(&mut characters_list) {
        let x = permutation.iter().collect();
        permutations.push(x);
    }

    permutations.sort();

    permutations
}

fn load_dictionary() -> Vec<String> {
    let mut dictionary: Vec<String> = Vec::new();

    let open_dict = File::open("words.txt");

    let dictionary_file = match open_dict {
        Ok(dict) => dict,
        Err(_) => panic!("Could not open file"),
    };

    let reader = BufReader::new(dictionary_file);

    for (_, line) in reader.lines().enumerate() {
        let word = match line {
            Ok(s) => s.to_lowercase(),
            Err(_) => "".to_string(),
        };

        if !word.is_empty() {
            dictionary.push(word);
        }
    }

    dictionary.sort();

    dictionary
}

fn main() {
    let mut input = String::new();

    loop {
        println!("{}", "Enter a word (MAX 10 letters): ".green());
        let res = io::stdin().read_line(&mut input);
        let input_size = input.trim().to_lowercase().len();

        match res {
            Ok(_) => {
                if input_size > 0 && input_size < 11 {
                    break;
                } else {
                    continue;
                }
            }
            Err(_) => panic!("Some error occurred!"),
        }
    }

    let binding = input.trim().to_lowercase();
    let word = binding.as_str();

    //Benchmark START
    let now = Instant::now();

    let dictionary: Vec<String> = load_dictionary();
    let combinations: Vec<String> = get_combinations(word);

    let mut permutations: Vec<String> = Vec::new();
    
    for combination in combinations {
       permutations.extend(get_permutations(&combination));
    }

    let shared_words: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let style = ProgressStyle::default_bar()
        .template("{msg} {bar:40} {pos}/{len}")
        .expect("Bar Style Error");

    permutations
        .par_iter()
        .progress_with_style(style)
        .for_each(|permutation| {
            let mut arr = shared_words.lock().unwrap();
                if dictionary.binary_search(permutation).is_ok() && !arr.contains(permutation) {
                    arr.push(permutation.to_string());
                }
        });

    let mut words = shared_words.lock().unwrap();
    
    words.sort();
    println!("{} {}", "Total words found:".cyan(), words.len());
    for word in words.iter() {
        println!("{}", word.cyan().italic().underline());
    }

    // //Benchmark END
    let elapsed = now.elapsed();
    println!("{} {:.2?}", "Time Elapsed:".yellow().bold(), elapsed);
}
