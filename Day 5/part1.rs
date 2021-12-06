use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let tokenized_input: Vec<&str> = input.split('\n').collect();

    let filtered_tokenized_input = tokenized_input.iter().take(tokenized_input.len() - 1)
        .map(| it | it.split(" -> ").collect::<Vec<&str>>())
        .map(| it | {
            let origin = it[0].split(',').collect::<Vec<&str>>();
            let destination = it[1].split(',').collect::<Vec<&str>>();
            
            return (
                (
                    origin[0].parse::<u32>().unwrap(),
                    origin[1].parse::<u32>().unwrap()
                ),
                (
                    destination[0].parse::<u32>().unwrap(),
                    destination[1].parse::<u32>().unwrap()
                )
            )
        })
        .filter(| it | it.0.0 != it.1.0 || it.0.1 != it.1.1)
        .collect::<Vec<((u32, u32), (u32, u32))>>();

    let horizontal_lines = filtered_tokenized_input.iter()
        .filter(| it | it.0.1 == it.1.1 )
        .collect::<Vec<&((u32, u32), (u32, u32))>>();

    let vertical_lines = filtered_tokenized_input.iter()
        .filter(| it | it.0.0 == it.1.0 )
        .collect::<Vec<&((u32, u32), (u32, u32))>>();

    let crossings = vertical_lines.iter()
        .map(
            |it| {
                let x = it.0.0;
                return horizontal_lines.iter()
                    .filter(| it2 | 
                        ((it2.0.0 <= x && it2.1.0 >= x ) || (it2.0.0 >= x && it2.1.0 <= x ) )
                        && 
                        ((it.0.1 >= it2.0.1 && it.1.1 <= it2.0.1) || (it.0.1 <= it2.0.1 && it.1.1 >= it2.0.1))
                    )
                    .map(|it2| (it.0.0, it2.0.1))
                    .collect::<Vec<(u32, u32)>>();
            }
        )
        .flatten()
        .collect::<Vec<(u32, u32)>>();

    let mut horizontal_coincidences = horizontal_lines.iter()
        .map(
            |it| {
                horizontal_lines.iter()
                    .filter(|it2| !(it.0.0 == it2.0.0 && it.0.1 == it2.0.1 && it.1.0 == it2.1.0 && it.1.1 == it.1.1))
                    .filter(|it2| it.0.1 == it2.0.1)
                    .map(|it2| find_common_points(&(it.0.0, it.1.0), &(it2.0.0, it2.1.0), &it.0.1, false))
                    .flatten()
                    .collect::<Vec<(u32, u32)>>()
            }
        )
        .flatten()
        .collect::<Vec<(u32, u32)>>();

        let mut vertical_coincidences = vertical_lines.iter()
        .map(
            | it | {
                vertical_lines.iter()
                    .filter(|it2| !(it.0.0 == it2.0.0 && it.0.1 == it2.0.1 && it.1.0 == it2.1.0 && it.1.1 == it.1.1))
                    .filter(|it2| it.0.0 == it2.0.0)
                    .map(|it2| find_common_points(&(it.0.1, it.1.1), &(it2.0.1, it2.1.1), &it.0.0, true))
                    .flatten()
                    .collect::<Vec<(u32, u32)>>()
            }
        )
        .flatten()
        .collect::<Vec<(u32, u32)>>();

        horizontal_coincidences.append(&mut vertical_coincidences);

        horizontal_coincidences.sort_by_key(|k| k.0);

        horizontal_coincidences.dedup();

        horizontal_coincidences.sort_by_key(|k| k.1);

        horizontal_coincidences.dedup();

        // any potential hiring people looking at this, know that it was 3am at the time and I was more than done with this.

        let non_coincident_crossing_points = crossings.iter()
            .filter(|it| !horizontal_coincidences.contains(it))
            .count();

        println!("{:?}", non_coincident_crossing_points + horizontal_coincidences.len());
}

fn find_common_points(bounds1: &(u32, u32), bounds2: &(u32, u32), constant: &u32, vertical: bool) -> Vec<(u32, u32)> {
    let upperBound1 = if bounds1.0 > bounds1.1 { bounds1.0 } else { bounds1.1 };
    let lowerBound1 = if bounds1.0 > bounds1.1 { bounds1.1} else { bounds1.0};

    let upperBound2 = if bounds2.0 > bounds2.1 { bounds2.0 } else { bounds2.1 };
    let lowerBound2 = if bounds2.0 > bounds2.1 { bounds2.1} else { bounds2.0};

    let mut result: Vec<(u32, u32)> = Vec::new();

    if lowerBound1 <= lowerBound2 && upperBound1 <= upperBound2 {
        if upperBound1 >= lowerBound2 {
            for num in lowerBound2..(upperBound1 + 1) {
                let point = if vertical { (*constant, num) } else { (num, *constant) };
                result.push(point);
            }
        } else {
            return result;
        }
    } else if lowerBound2 <= lowerBound1 && upperBound2 <= upperBound1 {
        if upperBound2 >= lowerBound1 {
            for num in lowerBound1..(upperBound2 + 1) {
                let point = if vertical { (*constant, num) } else { (num, *constant) };
                result.push(point);
            }
        } else {
            return result;
        }
    } else if lowerBound1 <= lowerBound2 && upperBound1 >= upperBound2 {
        for num in lowerBound2..(upperBound2 + 1) {
            let point = if vertical { (*constant, num) } else { (num, *constant) };
            result.push(point);
        }
    } else if lowerBound2 <= lowerBound1 && upperBound2 >= upperBound1 {
        for num in lowerBound1..(upperBound1 + 1) {
            let point = if vertical { (*constant, num) } else { (num, *constant) };
            result.push(point);
        }
    } else {
        return result;
    }

    return result;
}
