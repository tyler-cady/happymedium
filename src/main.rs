use rand::Rng;  // This is the correct import for the `gen_range` method.
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self};
use std::io::BufRead;


fn feel(value: &str, smap: &HashMap<String, f32>) -> i8 {
    let words = value.split(" ");
    let mut net_score: i8 = 0;

    for w in words {
        let k = *smap.get(w).unwrap_or(&0.0);  // Use f32 instead of i8 for `k`
        
        // Check using floating-point comparisons
        if k >= 0.05 {
            net_score += 1;
        } else if k > -0.05 && k < 0.05 {
            net_score += 0;
        } else {
            net_score -= 1;
        }
    }

    if net_score >= 1 {
        return 1;
    } else if net_score <= -1 {
        return -1;
    } else {
        return 0;
    }
}

fn read_in_sentiments(file_path: &str) -> HashMap<String, f32> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut smap: HashMap<String, f32> = HashMap::new();

    for line in contents.lines() {
        let mut parts = line.split('\t');
        if let (Some(word), Some(score)) = (parts.next(), parts.next()) {
            if let Ok(score) = score.parse::<f32>() {
                smap.insert(word.to_string(), score);
            }
        }
    }
    smap
}

fn choose_random_line(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    
    if lines.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "File is empty"));
    }
    
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..lines.len());
    
    Ok(lines[random_index].clone())
}

fn medium(score: i8) -> String {
    if score == 1 {
        // println!("Insult");
        return choose_random_line("src/Insults.txt").unwrap_or_else(|_| "Error reading file.".to_string());
    } 
    else if score == -1 {
        // println!("Compliment");
        return choose_random_line("src/Compliments.txt").unwrap_or_else(|_| "Error reading file.".to_string());
    } 
    else {
        // println!("Neutrality"); 
        return choose_random_line("src/Medium-isms.txt").unwrap_or_else(|_| "Error reading file.".to_string());
    }
}

fn main() {
    let file_path = "src/vaderlexw.txt";
    let smap: HashMap<String, f32> = read_in_sentiments(file_path);
    
    let stdin = io::stdin();
    

    println!("How are you feeling?");
    let mut input = String::new();

    input.clear();
    stdin.read_line(&mut input).expect("Failed to read line");
    println!("{}", medium(feel(input.trim(), &smap)));

}
