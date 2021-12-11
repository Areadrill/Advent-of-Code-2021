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

    let error_points: HashMap<char, u32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),]);


    let result = line_separation.iter()
        .map(|it| {
            let mut stack: Vec<char> = Vec::new();
            let mut points = 0;

            for ch in it.chars().collect::<Vec<char>>() {
                if expected_closers.values().collect::<Vec<&char>>().contains(&&ch) {
                    let popped = stack.pop().unwrap();

                    if *expected_closers.get(&popped).unwrap() != ch {
                        points = *error_points.get(&ch).unwrap();
                        break;
                    }
                } else {
                    stack.push(ch);
                }
            }

            return points;
        })
        .sum::<u32>();

    println!("{:?}", result);
}
