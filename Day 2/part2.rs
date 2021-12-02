use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let mut tokenized_inputs: Vec<&str> = input.split('\n').collect();
    tokenized_inputs.remove(tokenized_inputs.len() - 1);

    let mut depth: i32 = 0;
    let mut pos: i32 = 0;
    let mut aim: i32 = 0;

    for instruction in tokenized_inputs {
        let tokenized_instruction: Vec<&str> = instruction.split(' ').collect();

        let value = tokenized_instruction[1].parse::<i32>().unwrap();

        match tokenized_instruction[0] {
            "forward" => { pos += value; depth += aim * value },
            "up" => aim -= value,
            "down" => aim += value,
            _ => println!("impossible case happened o_O"),
        }
    }

    let result = depth * pos;

    println!("{}", result);
}
