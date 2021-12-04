use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let mut tokenized_inputs: Vec<&str> = input.split('\n').collect();
    tokenized_inputs.remove(tokenized_inputs.len() - 1);

    let occurrences = tokenized_inputs.iter().fold(
        vec![0; tokenized_inputs[0].len()],
        |mut acc, input| {
            let tokenized_binary: Vec<char> = input.chars().collect();

            for (idx, it) in tokenized_binary.iter().enumerate() {
                acc[idx] += it.to_string().parse::<i32>().unwrap();
            }

            return acc;
    });

    let gamma: Vec<u8> = occurrences.iter().map(|&it| if (it as f32) / (tokenized_inputs.len() as f32) >= 0.5 { 1 } else { 0 } ).collect();
    let str_gamma: Vec<String> = gamma.iter().map(|&it| it.to_string()).collect();
    let actual_gamma = str_gamma.join("");

    let str_epsilon: Vec<&str> = str_gamma.iter().map(|it|  if it == "1" { "0" } else { "1" }).collect();
    let actual_epsilon = str_epsilon.join("");

    let decimal_gamma = isize::from_str_radix(&actual_gamma, 2).unwrap();
    let decimal_epsilon = isize::from_str_radix(&actual_epsilon, 2).unwrap();

    println!("{:?}", actual_gamma);
    println!("{:?}", actual_epsilon);
    println!("{:?}", decimal_gamma*decimal_epsilon);
}
