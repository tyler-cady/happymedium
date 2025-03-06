use rand::Rng;  
use std::collections::HashMap;
use std::io::{self};
use include_dir::include_dir;

static ASSETS: include_dir::Dir = include_dir!("$CARGO_MANIFEST_DIR/src/assets");

fn feel(value: &str, smap: &HashMap<String, f32>) -> i8 {
    let words = value.split(" ");
    let mut net_score: i8 = 0;

    for w in words {
        let k = *smap.get(w).unwrap_or(&0.0);  
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
    let file = ASSETS.get_file(file_path).expect("File not found in assets directory");
    let contents = file.contents_utf8().expect("File is not valid UTF-8");

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
    let file = ASSETS.get_file(file_path).expect("File not found in assets directory");
    let contents = file.contents_utf8().unwrap();
    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

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
        return choose_random_line("Insults.txt").unwrap_or_else(|_| "Error reading file.".to_string());
    } 
    else if score == -1 {
        // println!("Compliment");
        return choose_random_line("Compliments.txt").unwrap_or_else(|_| "Error reading file.".to_string());
    } 
    else {
        // println!("Neutrality"); 
        return choose_random_line("Medium-isms.txt").unwrap_or_else(|_| "Error reading file.".to_string());
    }
}

fn main() {
    let file_path = "vaderlexw.txt";
    let smap: HashMap<String, f32> = read_in_sentiments(file_path);
    
    let stdin = io::stdin();
    

    println!("How are you feeling?");
    let mut input = String::new();

    input.clear();
    stdin.read_line(&mut input).expect("Failed to read line");
    println!("{}", medium(feel(input.trim(), &smap)));

}
