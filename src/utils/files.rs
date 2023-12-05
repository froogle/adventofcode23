use std::{io::{Error, Read}, fs::File};


/// Read a file and return its content as a string.
pub fn read_file(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Read a file and return a vector of strings, one for each line
pub fn read_strings(path: &str) -> Result<Vec<String>, Error> {
    let  content = read_file(path)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}

/// Read a file and return a vector of strings, but replacing a a specified character with something
pub fn read_strings_replace(path: &str, replace: &str, with: &str) -> Result<Vec<String>, Error> {
    let  content = read_file(path)?;
    let regex = regex::Regex::new(replace).unwrap();
    Ok(content.lines().map(|s| regex.replace_all(s, with).into_owned()).collect())
}

/// Read a file and parse each line as an integer, returning a vector of all integers
pub fn read_ints(path: &str) -> Result<Vec<i32>, Error> {
    let  content = read_file(path)?;
    Ok(content
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect())
}

/// Read a file of strings. Read up to each blank line into a vector of strings. Return
/// a vector or all the vectors
pub fn read_groups_strings(path: &str) -> Result<Vec<Vec<String>>, Error> {
    let  content = read_file(path)?;
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for line in content.lines() {
        if line.is_empty() {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(line.to_string());
        }
    }
    groups.push(group);
    Ok(groups)
}

/// Read a file of strings. Read up to each blank line into a vector of integers. Return
/// a vector of all the vectors of integers.
pub fn read_groups_ints(path: &str) -> Result<Vec<Vec<i32>>, Error> {
    let content = read_file(path)?;
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for line in content.lines() {
        if line.is_empty() {
            groups.push(group);
            group = Vec::new();
        } else {
            group.push(line.parse::<i32>().unwrap());
        }
    }
    groups.push(group);
    Ok(groups)
}

/// Read a file of strings, ignoring any lines matching a regex. Read up to each blank line into a vector of integers. Return
/// a vector of all the vectors of integers.
pub fn read_groups_ints_with_ignore(path: &str, ignore: &str) -> Result<Vec<Vec<i64>>, Error> {
    let content = read_file(path)?;
    let mut groups = Vec::new();
    let mut group = Vec::new();
    let regex = regex::Regex::new(ignore).unwrap();
    for line in content.lines() {

        if regex.is_match(line) {
            continue;
        }

        if line.is_empty() && !group.is_empty() {
            groups.push(group);
            group = Vec::new();
        } else {
            for number in line.split_whitespace() {
                group.push(number.parse::<i64>().unwrap());
            }
        }
    }
    groups.push(group);
    Ok(groups)
}
