use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let mut initial_vec = input.split('\n').collect::<Vec<&str>>()[0]
        .split(',').collect::<Vec<&str>>().iter()
        .map(|it| it.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    initial_vec.sort();

    let midpoint = initial_vec.len() as f32 / 2 as f32;

    let median: Vec<u32> = if initial_vec.len() % 2 == 0 {
        vec![initial_vec[midpoint.floor() as usize], initial_vec[midpoint.ceil() as usize]]
    } else {
        vec![initial_vec[midpoint as usize]]
    };

    let result = median.iter()
        .map(|it| initial_vec.iter()
            .map(|it2| (*it2 as i32 - *it as i32).abs())
            .sum::<i32>())
        .min()
        .unwrap();

    println!("{}", result);
}
