use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let initial_vec = input.split('\n').collect::<Vec<&str>>()[0]
        .split(',').collect::<Vec<&str>>().iter()
        .map(|it| it.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
        
    let sum_of_elements = initial_vec.iter().sum::<u32>();

    let mean = sum_of_elements as f32 / initial_vec.len() as f32;

    let result = vec![mean.floor(), mean.ceil()].iter()
        .map(|it| initial_vec.iter()
            .map(|it2| {
                let number_of_steps = (*it2 as i32 - *it as i32).abs() as u32;
                (0..number_of_steps).collect::<Vec<u32>>().iter().sum::<u32>() + number_of_steps
            })
            .sum::<u32>()
        )
        .min()
        .unwrap();


    println!("{}", result);
}
