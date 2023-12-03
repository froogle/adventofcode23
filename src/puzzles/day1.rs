use crate::utils;


pub fn part1() {
    println!("Advent 23 part 1");
    let input_lines = utils::files::read_strings("inputs/23_day_1.txt").unwrap();
    let mut total = 0;

    for line in &input_lines {
        let num = build_number(line);
        total += num;
    }
    println!("The total is {}", total);
}

pub fn part2() {
    println!("Advent 23 part 2");
    let mut total = 0;

    let input_lines = utils::files::read_strings("inputs/23_day_1.txt").unwrap();

    let replacements = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    for input in &input_lines {

        let mut line = "".to_string();
        
        for (input_position, input_character) in input.chars().enumerate() {
            if input_character.is_ascii_digit() {
                line = format!("{}{}", line, input_character);
            } else {
                for (replacement, replacement_value) in &replacements {
                    if input[input_position..].starts_with(replacement) {
                        line = format!("{}{}", line, replacement_value);
                    }
                }
            }
        }
        let num = build_number(&line);
        total += num;
    }
    println!("The total is {}", total);
}


fn build_number(line: &String) -> i32 {
    let first_digit = line.chars()
    .find(|c| c.is_ascii_digit()).unwrap();

    let last_digit = line.chars()
    .rfind(|c| c.is_ascii_digit()).unwrap();

    let num = format!("{}{}", first_digit, last_digit)
    .parse::<i32>()
    .expect("oops");
    num
}