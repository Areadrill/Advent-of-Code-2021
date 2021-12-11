use std::fs;
use std::collections::HashMap;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let line_separation = input.split('\n').collect::<Vec<&str>>();

    let expected_closers: HashMap<char, char> = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ]);

    let char_points: HashMap<char, u64> = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4)
    ]);

    let mut result = line_separation.iter()
        .take(line_separation.len() - 1)
        .filter(|it| {
            let mut stack: Vec<char> = Vec::new();

            for ch in it.chars().collect::<Vec<char>>() {
                if expected_closers.values().collect::<Vec<&char>>().contains(&&ch) {
                    let popped = stack.pop().unwrap();

                    if *expected_closers.get(&popped).unwrap() != ch {
                        return false
                    }
                } else {
                    stack.push(ch);
                }
            }

            return true;
        })
        .map(|it| {
            let mut initial_stack: Vec<char> = Vec::new();

           let mut missing_part =  it.chars().fold(initial_stack, |mut accum, it2| {
                if expected_closers.values().collect::<Vec<&char>>().contains(&&it2) {
                    accum.pop();
                } else {
                    accum.push(it2);
                }

                accum
            });

            missing_part.reverse();

            missing_part

        })
        .map(|it| it.iter().fold(0 as u64, |accum, it| (5 as u64*accum as u64) + *char_points.get(it).unwrap()))
        .collect::<Vec<u64>>();

    result.sort();
    
    println!("{:?}", result[(result.len() - 1)/ 2 ]);
}
