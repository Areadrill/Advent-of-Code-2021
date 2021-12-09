use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let separation = input.split('\n').collect::<Vec<&str>>().iter()
        .map(|it| it.split(" | ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let payloads = separation.iter()
        .take(separation.len() - 1)
        .map(|it| it[1].split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let result = payloads.iter()
        .map(|it| it.iter()
            .filter(|it2| it2.len() == 2 || it2.len() == 4 || it2.len() == 3 || it2.len() == 7)
            .count() as u32
        )
        .sum::<u32>();
    
    
    println!("{:?}", result);
}
