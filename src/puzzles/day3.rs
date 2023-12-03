use crate::utils;


pub fn part2() { 

    println!("Advent 23, day 3 part 2");
    let mut total = 0;

    let input_lines = utils::files::read_strings_replace("inputs/23_day_3.txt", r"[^\d*]", " ").unwrap();

    for (i, line) in input_lines.iter().enumerate() {        
        for cap in regex::Regex::new(r"\*").unwrap().captures_iter(line) {
            for mat in cap.iter() { 
                if let Some(m) = mat {
                       
                    let mut grid = String::new();
                    let grid_start  = m.start().saturating_sub(1); 
                    let grid_end = m.end(); //m.end().saturating_add(1).min(input_lines[i].len());
     
                    if i > 0 {             
                        grid.push_str(build_grid_line(grid_start, &input_lines[i-1], grid_end));
                    }
                 
                    grid.push_str(build_grid_line(grid_start, &input_lines[i], grid_end));
   
                    if i < input_lines.len() - 1 {
                        grid.push_str(build_grid_line(grid_start, &input_lines[i+1], grid_end));
                    }

                    let count = regex::Regex::new(r"\d+").unwrap().find_iter(&grid).count();
                    if count == 2 {
                        let mut product = 1;
                        for cap in regex::Regex::new(r"\d+").unwrap().captures_iter(&grid) {
                            for mat in cap.iter() { 
                                if let Some(m) = mat {
                                    product *= m.as_str().parse::<i32>().unwrap();
                                }
                            }
                        }
                        total += product;
                    }
                }
            }
        }
    }

    println!("The answer is {}", total);
    
}


pub fn part1() {
    println!("Advent 23, day 3 part 1");

    let mut total = 0;

    let input_lines = utils::files::read_strings_replace("inputs/23_day_3.txt", r"\.", " ").unwrap();

    // iterate over the lines in input_lines with an index
    for (i, line) in input_lines.iter().enumerate() {        
        
        for cap in regex::Regex::new(r"\d+").unwrap().captures_iter(line) {
            for mat in cap.iter() { 
                if let Some(m) = mat {
                       
                    let mut grid = String::new();
                    let grid_start  = m.start().saturating_sub(1); 
                    let grid_end = m.end().saturating_add(1).min(input_lines[i].len());
     
                    if i > 0 {
                        grid.push_str(&input_lines[i-1][grid_start..grid_end]);
                    }
                    
                    grid.push_str(&input_lines[i][grid_start..grid_end]);
                    
                    if i < input_lines.len() - 1 {
                        grid.push_str(&input_lines[i+1][grid_start..grid_end]);
                    }

                    let symbol_count = grid.chars().filter(|&c| c != ' ' && !c.is_numeric()).count();
                    if symbol_count > 0 {
                        total += cap[0].to_string().parse::<i32>().unwrap();
                    }
                }
            }
        }
    }

    println!("The answer is {}", total);
}



fn build_grid_line(mut grid_start: usize, input_line: &String, mut grid_end: usize) -> &str {
    while grid_start > 0 && input_line.as_bytes()[grid_start].is_ascii_digit() {
        grid_start -= 1;
    }
                    
    while grid_end < input_line.len() && input_line.as_bytes()[grid_end].is_ascii_digit() {
        grid_end += 1;
    }
    let grid_string = &input_line[grid_start..grid_end];
    grid_string
}