use std::fs;
use std::process;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let tokenized_inputs: Vec<&str> = input.split('\n').collect();

    if tokenized_inputs.len() < 2 {
        println!("0");
        process::exit(0);
        
    }

    let mut result = 0;

    for reading_idx in 1..tokenized_inputs.len() - 1  {
        if tokenized_inputs[reading_idx].parse::<i32>().unwrap() > tokenized_inputs[reading_idx - 1].parse::<i32>().unwrap() {
            result += 1;
        }
    }

    println!("{}", result);
}
