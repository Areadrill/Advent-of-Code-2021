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

    let alphabet = separation.iter()
        .take(separation.len() - 1)
        .map(|it| it[0].split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let decoded_alphabets = &alphabet.iter()
        .map(|it| decode_alphabet(&it).iter()
            .map(|it| sort_string(it))
            .collect::<Vec<String>>()
        )
        .collect::<Vec<Vec<String>>>();
        
    let result: u32 = payloads.iter()
        .map(|it| it.iter().map(|it2| sort_string(it2)).collect::<Vec<String>>())
        .enumerate()
        .map(|(idx, it)| it.iter()
            .map(|it2| {
                decoded_alphabets[idx].iter()
                    .position(|it3| it3 == it2).unwrap() as u32
            })
            .collect::<Vec<u32>>())
        .map(|it| it[0] * 1000 + it[1] * 100 + it[2] * 10 + it[3])
        .sum();

    println!("{:?}", result);
}

fn sort_string(to_sort: &str) -> String{
    let mut breakdown = to_sort.chars().collect::<Vec<char>>();
                
    breakdown.sort(); 

    return breakdown.iter().collect::<String>();
}

fn decode_alphabet<'a>(encoded: &Vec<&str>) -> Vec<String> {
    let mut result = [""; 10];

    result[1] = encoded.iter().filter(|it| it.len() == 2).map(|it| *it).collect::<Vec<&str>>()[0];
    result[4] = encoded.iter().filter(|it| it.len() == 4).map(|it| *it).collect::<Vec<&str>>()[0];
    result[7] = encoded.iter().filter(|it| it.len() == 3).map(|it| *it).collect::<Vec<&str>>()[0];
    result[8] = encoded.iter().filter(|it| it.len() == 7).map(|it| *it).collect::<Vec<&str>>()[0];

    result[9] = encoded.iter()
        .filter(|it| it.len() == 6 && result[4].chars().collect::<Vec<char>>().iter()
            .filter(|it2| it.chars().collect::<Vec<char>>().contains(it2))
            .count() == 4
        )
        .map(|it| *it)
        .collect::<Vec<&str>>()[0];

    result[0] = encoded.iter()
        .filter(|it| it.len() == 6 && result[1].chars().collect::<Vec<char>>().iter()
            .filter(|it2| it.chars().collect::<Vec<char>>().contains(it2))
            .count() == 2 && **it != result[9]
        )
        .map(|it| *it)
        .collect::<Vec<&str>>()[0];

    result[6] = encoded.iter().filter(|it| it.len() == 6 && **it != result[9] && **it != result[0]).map(|it| *it).collect::<Vec<&str>>()[0];

    result[3] = encoded.iter()
        .filter(|it| it.len() == 5 && result[1].chars().collect::<Vec<char>>().iter()
            .filter(|it2| it.chars().collect::<Vec<char>>().contains(it2))
            .count() == 2
        )
        .map(|it| *it)
        .collect::<Vec<&str>>()[0];

    result[5] = encoded.iter()
        .filter(|it| it.len() == 5 && it.chars().collect::<Vec<char>>().iter()
            .filter(|it2| result[9].chars().collect::<Vec<char>>().contains(it2))
            .count() == 5 && **it != result[3]
    )
    .map(|it| *it)
    .collect::<Vec<&str>>()[0];

    result[2] = encoded.iter().filter(|it| it.len() == 5 && **it != result[3] && **it != result[5]).map(|it| *it).collect::<Vec<&str>>()[0];

    return result.iter().map(|it| it.to_string()).collect::<Vec<String>>();
}
