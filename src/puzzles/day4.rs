use crate::utils;
use std::collections::HashMap;

pub fn part1() {
  println!("Advent 23, day 4");
  let mut total = 0 ;

  let input_lines = utils::files::read_strings_replace("inputs/23_day_4.txt", r"Card\s+\d+: ", "").unwrap();

  let mut card_counts = HashMap::new();
  for i in 0..input_lines.len() {
    card_counts.insert(i.to_string(), 1);
  }

  for (i, card) in input_lines.iter().enumerate() {

    let mut matches = 0 ;

    let lists = card.split("|").collect::<Vec<&str>>();
    let mut winning_numbers = lists[0].split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    winning_numbers.sort_unstable();

    let mut my_numbers = lists[1].split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    my_numbers.sort_unstable();

    for number in my_numbers {
      if winning_numbers.contains(&number) {
        matches += 1;
      }
    }

    for duplicates_index in i+1..i+1+matches {
      //increase each card this card's counts number of times
       card_counts.insert(duplicates_index.to_string(), card_counts[&duplicates_index.to_string()] + card_counts[&i.to_string()]);
    }


    if matches > 0 {
      total += (1 << matches-1) as i32;
    }
  }
  println!("The answer to part 1 is {}", total);

  let card_counts_values_total = card_counts.values().sum::<i32>();
  println!("The answer to part 2 is {}",card_counts_values_total );


}

pub fn part2() {}
