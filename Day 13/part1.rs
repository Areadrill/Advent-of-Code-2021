use std::fs;
use std::collections::HashMap;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let section_separation = input.split("\n\n").collect::<Vec<&str>>();

    let dot_matrix = section_separation[0];
    let folding_instructions = section_separation[1].split('\n').collect::<Vec<&str>>();

    let mut dot_map: HashMap<i32, Vec<i32>> = HashMap::new();

    dot_matrix.split('\n').collect::<Vec<&str>>().iter()
        .for_each(|it| {
            let coords = it.split(',').collect::<Vec<&str>>();

            let y = coords[1].parse::<i32>().unwrap();
            let x = coords[0].parse::<i32>().unwrap();

            if dot_map.get(&y) == None {
                dot_map.insert(y, Vec::new());
            }

            dot_map.get_mut(&y).unwrap().push(x);
        });

        let folds = folding_instructions.iter()
            .take(folding_instructions.len() - 1)
            .map(|it| it.split(' ').collect::<Vec<&str>>()[2])
            .map(|it| it.split('=').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        folds.iter().for_each(|it| {
            dot_map = if it[0] == "x" {
                fold_x(it[1].parse::<i32>().unwrap(), &dot_map)
            } else {
                fold_y(it[1].parse::<i32>().unwrap(), &dot_map)
            };
        });

        print_map(&dot_map);
}

fn fold_y(line: i32, dots: &HashMap<i32, Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut result = dots.clone();
    
    dots.keys().collect::<Vec<&i32>>().iter()
        .filter(|it| ***it > line)
        .for_each(|it| {
            let target_row = (line - 1) - (*it - (line + 1));

            if dots.get(&target_row) == None {
                result.insert(target_row, Vec::new());
            }

            dots.get(it).unwrap().iter().
                for_each(|it2| {
                    if !result.get(&target_row).unwrap().contains(it2) {
                        result.get_mut(&target_row).unwrap().push(*it2);
                    }
                });

            result.remove(it);
        });

    return result;
}

fn fold_x(line: i32, dots: &HashMap<i32, Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut result = dots.clone();
    
    dots.keys().collect::<Vec<&i32>>().iter()
        .for_each(|it| {
            dots.get(it).unwrap().iter()
                .filter(|it2| **it2 > line)
                .for_each(|it2| {
                    let target_column = (line - 1) - (*it2 - (line + 1));

                    if !dots.get(it).unwrap().contains(&target_column) {
                        result.get_mut(it).unwrap().push(target_column);
                    }

                    let remove_idx = result.get_mut(it).unwrap().iter().position(|it3| it3 == it2).unwrap();
                    result.get_mut(it).unwrap().remove(remove_idx);
                });
        });

    return result;
}

fn print_map(dot_map: &HashMap<i32, Vec<i32>>) {
    let max_row = dot_map.keys().collect::<Vec<&i32>>().iter().map(|it| **it).max().unwrap();

    let max_col = dot_map.keys().collect::<Vec<&i32>>().iter()
        .map(|it| dot_map.get(it).unwrap().iter().max().unwrap())
        .max()
        .unwrap();

    for y in 0..max_row+1 {
        let mut dots_for_line = Vec::new();

        if dot_map.get(&y) != None {
            let mut lol = dot_map.get(&y).unwrap().clone();
            dots_for_line.append(&mut lol);
        }

        for x in 0..*max_col+1 {
            if dots_for_line.contains(&x) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
