use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let initial_vec = input.split('\n').collect::<Vec<&str>>()[0]
        .split(',').collect::<Vec<&str>>().iter()
        .map(|it| it.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let mut initial_state = [0, 0, 0, 0, 0, 0, 0, 0, 0];

    initial_vec.iter().for_each(|it| initial_state[*it as usize] += 1);

    let mut current_state = initial_state.clone();

    for i in 0..256 {
        if i == 80 {
            println!("{}", current_state.iter().sum::<u64>());
        }

        current_state = step(&current_state);
    }

    println!("{}", current_state.iter().sum::<u64>());
}

fn step(state: &[u64; 9]) -> [u64; 9] {
    return [state[1], state[2], state[3], state[4], state[5], state[6], state[7] + state[0], state[8], state[0]];
}
