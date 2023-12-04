use crate::utils;

pub fn part1() {
    println!("Advent 23, day 2 part 1");
    let mut total = 0 ;
    let mut total2 = 0;

    let mut dict = std::collections::HashMap::new();
    dict.insert("red", 12);
    dict.insert("green", 13);
    dict.insert("blue", 14);


    let input_lines = utils::files::read_strings_replace("inputs/23_day_2.txt", r"Game \d+: ", "").unwrap();
    for (i, line) in input_lines.iter().enumerate() {
        let mut possible = true ;

        let mut game = std::collections::HashMap::new();
        game.insert("red", 0);
        game.insert("green", 0);
        game.insert("blue", 0);


        for round in line.split(';') {
            for value in round.split(',') {
                let parts: Vec<&str> = value.split_whitespace().collect();

                let count: i32 = parts[0].parse().unwrap_or(0);
                let colour = parts[1];

                if dict[colour] < count {
                    possible = false
                }

                if game[colour] < count {
                    game.insert( colour, count);
                }
            }
        }

        total2 += game["red"] * game["green"] * game["blue"];

        if possible {
            total += i+1;
        }
    }

    println!("The answer is {}", total);
    println!("The answer to the second part is {}", total2);
}

