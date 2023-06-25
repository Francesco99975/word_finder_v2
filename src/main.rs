use std::{io::{self, BufReader, BufRead}, fs::File, sync::{Mutex, Arc}, time::{Instant, Duration}};

use colored::Colorize;
use indicatif::{ProgressStyle, ParallelProgressIterator, ProgressBar};
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};

fn get_permutations(word: &str) -> Vec<String> {
    let now = Instant::now();

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    pb.set_message(format!("{}","Gathering all possible permutations...".green().italic()));
    
    let mut permutations: Vec<String> = Vec::new();
    let mut characters_list: Vec<char> = word.chars().collect();
        
    for permutation in permutohedron::Heap::new(&mut characters_list) {
        let x = permutation.iter().collect();
        permutations.push(x);
    }

    permutations.sort();

    let elapsed = now.elapsed();
    pb.finish_with_message(format!("{} {:.2?}", "Time Elapsed for permutations:".yellow().bold(), elapsed));

    return permutations;
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

    return dictionary;
}

fn main() {
    let mut input = String::new();
    
    loop {
        println!("{}", "Enter a word (MAX 10 letters): ".green());
        let res = io::stdin().read_line(&mut input);
        let input_size = input.trim().to_lowercase().len();

        match res {
            Ok(_) => if input_size > 0 &&  input_size < 11 { break; } else { continue; },
            Err(_) => panic!("Some error occurred!"),
        }   
    }

    let binding = input.trim().to_lowercase();
    let word = binding.as_str();

    //Benchmark START
    let now = Instant::now();

    let dictionary: Vec<String> = load_dictionary();
    
    let permutations: Vec<String> = get_permutations(word);
    
    // println!("Dictionary Length: {}", dictionary.len());
    // println!("Permutations Length: {}", permutations.len());

    let words: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let num_threads = num_cpus::get(); // Get the number of available threads
    // println!("Threads Num: {}", num_threads);

    let style = ProgressStyle::default_bar().template("{msg} {bar:40} {pos}/{len}").expect("Bar Style Error");

    let thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .expect("Cannot build Thread Pool");

    thread_pool.install(|| {
        dictionary.par_iter().progress_with_style(style).for_each(|line| {
            if permutations.binary_search(line).is_ok() {
                words.lock().unwrap().push(line.to_string());
            }
        });
    });
   

    let words_vector = words.lock().unwrap();
    println!("{} {}", "Total words found:".cyan(), words_vector.len());
    for word in &*words_vector {
        println!("{}", word.cyan().italic().underline());
    }

    // //Benchmark END
    let elapsed = now.elapsed();
    println!("{} {:.2?}", "Time Elapsed:".yellow().bold(), elapsed);

}
