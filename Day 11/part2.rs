use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let line_separation = input.split('\n').collect::<Vec<&str>>();

    let mut matrix = line_separation.iter()
        .take(line_separation.len()  - 1)
        .map(|it| it.chars().map(|it2| it2.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let mut cnt = 0;

    while matrix.iter().filter(|it| it.iter().filter(|it2| **it2 != 0).count() != 0).count() != 0 {
        cnt += 1;
        matrix = step(&matrix).1;
    }

    println!("{}", cnt);
    
}

fn step(matrix: &Vec<Vec<u8>>) -> (u32, Vec<Vec<u8>>) {
    let mut flashes: Vec<(u8, u8)> = Vec::new();
    let mut flash_accum = 0;

    let mut result = matrix.iter()
        .enumerate()
        .map(|(y, it)| it.iter()
            .enumerate()
            .map(|(x, it2)| {
                if it2 + 1 > 9 {
                    flashes.push((x as u8, y as u8));
                    return 0;
                }

                return it2 + 1;
            })
            .collect::<Vec<u8>>()
        ).collect::<Vec<Vec<u8>>>();

    for position in flashes {
        let iteration_result = process_flash(&position, &mut result);

        flash_accum += iteration_result.0;

        result = iteration_result.1;
    }

    return (flash_accum, result)
        
}

fn process_flash(position: &(u8, u8), matrix: &mut Vec<Vec<u8>>) -> (u32, Vec<Vec<u8>>) {
    let mut result = matrix.clone();
    let mut flash_accum = 1;
    let mut flashes: Vec<(u8, u8)> = Vec::new();

    get_neighbors(position).iter()
        .for_each(|(x, y)| {
            if result[*y as usize][*x as usize] + 1 > 9 {
                result[*y as usize][*x as usize] = 0;
                flashes.push((*x, *y));
            } else if result[*y as usize][*x as usize] == 0 {
                
            } else {
                result[*y as usize][*x as usize] = result[*y as usize][*x as usize] + 1;
            }
        });

    for position in flashes {
        let iteration_result = process_flash(&position, &mut result);

        flash_accum += iteration_result.0;

        result = iteration_result.1;
    }

    return (flash_accum, result);
}

fn get_neighbors(position: &(u8, u8)) -> Vec<(u8, u8)> {
    let mut result: Vec<(u8, u8)> = Vec::new();

    if position.0 != 0 {
        result.push((position.0 - 1, position.1));

        if position.1 != 0 {
            result.push((position.0 - 1, position.1 - 1));
        }

        if position.1 != 9 {
            result.push((position.0 - 1, position.1 + 1));
        }
    }

    if position.0 != 9 {
        result.push((position.0 + 1, position.1));

        if position.1 != 0 {
            result.push((position.0 + 1, position.1 - 1));
        }

        if position.1 != 9 {
            result.push((position.0 + 1, position.1 + 1));
        }
    }

    if position.1 != 0 {
        result.push((position.0, position.1 - 1));
    }

    if position.1 != 9 {
        result.push((position.0, position.1 + 1));
    }

    return result;
}
