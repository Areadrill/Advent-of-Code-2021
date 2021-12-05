use std::fs;

fn main() {

    let input = fs::read_to_string("./input.txt")
        .expect("Something went wrong reading the file");

    let tokenized_input: Vec<&str> = input.split('\n').collect();

    let draw_order = tokenized_input[0].split(',').collect::<Vec<&str>>();

    let mut bingo_arrays: Vec<Vec<Vec<&str>>> = Vec::new();

    let mut accum_horizontal_array: Vec<Vec<&str>> = Vec::new();
    let mut accum_vertical_array: Vec<Vec<&str>> = vec![vec![]; 5];

    for line in (&tokenized_input).iter().skip(2).map(|it| *it).collect::<Vec<&str>>() {
        if line == "" {
            accum_horizontal_array.append(& mut accum_vertical_array);
            bingo_arrays.push(accum_horizontal_array);
            accum_horizontal_array = Vec::new();
            accum_vertical_array = vec![vec![]; 5];
        } else {
            let tokenized_line = line.split_whitespace().collect::<Vec<&str>>();

            for (idx, it) in tokenized_line.iter().enumerate() {
                accum_vertical_array[idx].push(it);
            }
    
            accum_horizontal_array.push(tokenized_line);
        }
    }

    let earliest_wins = bingo_arrays.iter()
        .map(|it| find_earliest_win(&it, &draw_order))
        .collect::<Vec<i8>>();

    let winning_draw_idx = earliest_wins.iter().min().unwrap();
    let winning_draw_number = draw_order[*winning_draw_idx as usize];
    let winning_board_idx = earliest_wins.iter().position(|it| it == winning_draw_idx).unwrap();
    let winning_board = &bingo_arrays[winning_board_idx];

    let undrawn_winning_board_numbers: u32 = winning_board.iter()
        .take(5)
        .flatten()
        .filter(|it| !draw_order[0..((*winning_draw_idx + 1) as usize)].contains(it))
        .map(|it| it.parse::<u32>().unwrap())
        .sum();

    println!("{}", undrawn_winning_board_numbers * winning_draw_number.parse::<u32>().unwrap());
}

fn find_earliest_win(data: &Vec<Vec<&str>>, draw_order: &Vec<&str>) -> i8{
    return data.iter()
        .map(|it| it.iter()
            .map(|it2| draw_order.iter()
                .position(|it3| it3 == it2)
            )
            .map(|it2| it2.unwrap() as i8)
            .max()
            .unwrap()
        )
        .min()
        .unwrap()
}
