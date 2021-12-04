use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let mut tokenized_inputs: Vec<&str> = input.split('\n').collect();
    tokenized_inputs.remove(tokenized_inputs.len() - 1);

    let o2: u32 = get_reading(&tokenized_inputs, true);
    let co2: u32 = get_reading(&tokenized_inputs, false);    

    println!("{}", o2 * co2);
}

fn get_reading(data: &Vec<&str>, most_common: bool) -> u32 {
    let mut dup_data = data.clone();
    let mut occurrences = get_occurrences(&dup_data);

    for idx in 0..(&dup_data[0]).len() {
        if (occurrences[idx] as f32) / ((&dup_data).len() as f32) >= 0.5 {
            let thing: Vec<&&str> = dup_data.iter().filter(|&it| {
                let chars: Vec<char> = it.chars().collect();

                return chars[idx] == if most_common {'1'} else {'0'};

            }).collect();

            dup_data = thing.iter().map(|&it| &**it).collect();

            occurrences = get_occurrences(&dup_data);
        } else {
            let thing: Vec<&&str> = dup_data.iter().filter(|&it| {
                let chars: Vec<char> = it.chars().collect();

                return chars[idx] == if most_common {'0'} else {'1'};

            }).collect();

            dup_data = thing.iter().map(|&it| &**it).collect();

            occurrences = get_occurrences(&dup_data);
        }

        if dup_data.len() == 1 {
            return isize::from_str_radix(&dup_data[0], 2).unwrap() as u32;
        }
    }

    return 0;
}

fn get_occurrences(data: &Vec<&str>) -> Vec<i32> {
    return data.iter().fold(
        vec![0; data[0].len()],
        |mut acc, input| {
            let tokenized_binary: Vec<char> = input.chars().collect();

            for (idx, it) in tokenized_binary.iter().enumerate() {
                acc[idx] += it.to_string().parse::<i32>().unwrap();
            }

            return acc;
    });
}
