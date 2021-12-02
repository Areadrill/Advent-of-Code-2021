use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let mut tokenized_inputs: Vec<&str> = input.split('\n').collect();
    tokenized_inputs.remove(tokenized_inputs.len() - 1);

    let mut depth = 0;
    let mut pos = 0;

    for instruction in tokenized_inputs {
        let tokenized_instruction: Vec<&str> = instruction.split(' ').collect();

        let value = tokenized_instruction[1].parse::<u32>().unwrap();

        match tokenized_instruction[0] {
            "forward" => pos += value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => println!("impossible case happened o_O"),
        }
    }

    let result = depth * pos;

    println!("{}", result);
}
