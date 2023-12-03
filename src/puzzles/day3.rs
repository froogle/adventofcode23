use crate::utils;

pub fn part1() {
    println!("Advent 23, day 2 part 1");

    let mut total = 0;

    let input_lines = utils::files::read_strings_replace("inputs/23_day_3.txt", ".", " ").unwrap();


    // iterate over the lines in input_lines with an index
    for (i, line) in input_lines.iter().enumerate() {        
        
        for cap in regex::Regex::new(r"\d+").unwrap().captures_iter(line) {
            for mat in cap.iter() { 
                if let Some(m) = mat {

                    let actual_start  = m.start().saturating_sub(1); 
                    let actual_end = m.end().saturating_add(1).min(input_lines[i].len());
        
                    let mut grid = String::new();
        
                    if i > 0 {
                        grid.push_str(&input_lines[i-1][actual_start..actual_end]);
                    }
                    
                    grid.push_str(&input_lines[i][actual_start..actual_end]);
                    
                    if i < input_lines.len() - 1 {
                        grid.push_str(&input_lines[i+1][actual_start..actual_end]);
                    }
                    let non_space_or_number_count = grid.chars().filter(|&c| c != ' ' && !c.is_numeric()).count();
                    if non_space_or_number_count > 0 {
                        total += cap[0].to_string().parse::<i32>().unwrap();
                    }
                }
            }
        }
    }

    println!("THE ANSWER TO DAY 3 PART 1 IS {}", total);
}
