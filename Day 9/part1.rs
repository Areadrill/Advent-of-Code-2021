use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let line_separation = input.split('\n').collect::<Vec<&str>>();

    let matrix = line_separation.iter()
        .take(line_separation.len() - 1)
        .map(|it| it.chars().map(|it2| it2.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let result = matrix.iter()
        .enumerate()
        .map(|(y, it)| it.iter()
            .enumerate()
            .filter(|(x, it2)| get_neighbors(&(*x as u8, y as u8)).iter()
                .map(|it3| matrix[it3.1 as usize][it3.0 as usize])
                .min()
                .unwrap() > **it2)
            .map(|(_, it2)| *it2)
            .collect::<Vec<u8>>()
        )
        .map(|it| it.len() as u32 + it.iter().map(|it2| *it2 as u32).sum::<u32>() )
        .sum::<u32>();
    
    println!("{}", result);

}

fn get_neighbors(position: &(u8, u8)) -> Vec<(u8, u8)> {
    let mut result: Vec<(u8, u8)> = Vec::new();

    if position.0 != 0 {
        result.push((position.0 - 1, position.1));
    }

    if position.0 != 99 {
        result.push((position.0 + 1, position.1));
    }

    if position.1 != 0 {
        result.push((position.0, position.1 - 1));
    }

    if position.1 != 99 {
        result.push((position.0, position.1 + 1));
    }

    return result;
}
