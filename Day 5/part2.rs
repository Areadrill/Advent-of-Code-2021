use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let tokenized_input: Vec<&str> = input.split('\n').collect();

    let mut points = tokenized_input.iter().take(tokenized_input.len() - 1)
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
        .map(|it| generate_points(&it.0, &it.1))
        .flatten()
        .collect::<Vec<(u32, u32)>>();

        points.sort_by(|a, b| {
            if a.0 != b.0 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });

        let mut repeated_count = 0;

        let mut prev_repeater: (u32, u32) = (std::u32::MAX, std::u32::MAX);

        let mut prev: (u32, u32) = (std::u32::MAX, std::u32::MAX);

        for point in points {
            if point == prev && prev_repeater != point {
                repeated_count += 1;
                prev_repeater = point;
            }

            prev = point;
        }

        println!("{}", repeated_count);
}

fn generate_points(origin: &(u32, u32), destination: &(u32, u32)) -> Vec<(u32, u32)> {
    let mut result: Vec<(u32, u32)> = Vec::new();

    let aux_vec = vec![origin.1, destination.1];
    let aux_vec_2 = vec![origin.0, destination.0];

    if origin.0 == destination.0 {
        let lower_bound = aux_vec.iter().min().unwrap();
        let upper_bound = aux_vec.iter().max().unwrap();

        for num in *lower_bound..(upper_bound + 1) {
            result.push((origin.0, num));
        }
    } else if origin.1 == destination.1 {
        let lower_bound = aux_vec_2.iter().min().unwrap();
        let upper_bound = aux_vec_2.iter().max().unwrap();

        for num in *lower_bound..(upper_bound + 1) {
            result.push((num, origin.1));
        }
    } else {
        let upper_point = if origin.1 < destination.1 {origin} else {destination};
        let lower_point = if origin.1 < destination.1 {destination} else {origin};

        if upper_point.0 > lower_point.0 {
            for num in 0..(lower_point.1 - upper_point.1 + 1) {
                result.push((upper_point.0 - num, upper_point.1 + num ));
            }
        } else {
            for num in 0..(lower_point.1 - upper_point.1 + 1) {
                result.push((upper_point.0 + num, upper_point.1 + num ));
            }
        }
    }

    return result;
}
