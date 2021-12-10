use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let line_separation = input.split('\n').collect::<Vec<&str>>();

    let matrix = line_separation.iter()
        .take(line_separation.len() - 1)
        .map(|it| it.chars().map(|it2| it2.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let mut result = matrix.iter()
        .enumerate()
        .map(|(y, it)| it.iter()
            .enumerate()
            .filter(|(x, it2)| get_neighbors(&(*x as u8, y as u8)).iter()
                .map(|it3| matrix[it3.1 as usize][it3.0 as usize])
                .min()
                .unwrap() > **it2)
            .map(|(x, _)| explore_basin(&(x as u8, y as u8), &matrix, &mut vec![(x as u8, y as u8)]).len() as u32)
            .collect::<Vec<u32>>()
        )
        .flatten()
        .collect::<Vec<u32>>();

        result.sort();
        result.reverse();
    
    println!("{:?}", result[0] * result[1] * result[2]);

}

fn explore_basin(position: &(u8, u8), map: &Vec<Vec<u8>>, visited: &mut Vec<(u8, u8)>) -> Vec<(u8, u8)> {
    if map[position.1 as usize][position.0 as usize] == 9 {
        return visited.to_vec();
    }

    let mut result: Vec<(u8, u8)> = Vec::new();

    visited.push((position.0, position.1));

    if position.0 != 0 && !visited.contains(&(position.0 - 1, position.1)) {
        result.append(&mut explore_basin(&(position.0 - 1, position.1), map, visited));
    }

    if position.0 != 99 && !visited.contains(&(position.0 + 1, position.1)) {
        result.append(&mut explore_basin(&(position.0 + 1, position.1), map, visited));
    }

    if position.1 != 0 && !visited.contains(&(position.0, position.1 - 1)) {
        result.append(&mut explore_basin(&(position.0, position.1 - 1), map, visited));
    }

    if position.1 != 99 && !visited.contains(&(position.0, position.1 + 1)) {
        result.append(&mut explore_basin(&(position.0, position.1 + 1), map, visited));
    }

    result.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    result.dedup();

    return result;
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
