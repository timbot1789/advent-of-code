use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Cannot read file");
    let result : i32 = find_max(&contents);

    println!("heaviest elf: {result}");
}

fn find_max(contents: &str) -> i32 {
    let parts = contents.split('\n');
    let mut max_weight: i32 = 0;
    let mut cur_weight: i32 = 0;

    for part in parts {
        if part == "" {
            if cur_weight > max_weight {max_weight = cur_weight}
            cur_weight = 0;
        } else {
            cur_weight += part.parse::<i32>().unwrap();
        }
    }

    max_weight
}
