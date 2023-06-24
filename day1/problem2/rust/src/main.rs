use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Cannot read file");
    let result : isize = find_max(&contents);

    println!("heaviest elf: {result}");
}

fn find_max(contents: &str) -> isize {
    let parts = contents.split('\n');
    let mut cur_weight: isize = 0;
    let mut weights: Vec<isize> = vec![];

    for part in parts {
        if part == "" {
            weights.push(cur_weight);
            cur_weight = 0;
        } else {
            cur_weight += part.parse::<isize>().unwrap();
        }
    }

    weights.sort_by(|a,b| b.cmp(a));

    weights[0] + weights[1] + weights[2]
}
