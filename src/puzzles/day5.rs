use crate::utils;

pub fn part1() {
  println!("Advent 23, day 5");
  let input_lines = utils::files::read_groups_ints_with_ignore("inputs/23_day_5.txt", r"[^:]+:").unwrap();

  let mut locations = Vec::new();

  let mut source:i64 = -1;
  let mut destination:i64 = -1;
  let mut count = -1;

  for group in input_lines {

    if locations.is_empty() {
      build_initial_seed_list(&group, &mut locations);
      continue;
    }

    let mut final_destinations = Vec::new();
    let mut removals = Vec::new();


    for value in group {
      if destination == -1 { destination = value; continue; }
      if source == -1 { source = value; continue; }
      if count == -1 {
        count = value;

        for (i, source_location) in locations.clone().iter().enumerate() {
          check_seed_location(source, count, source_location, &mut removals, i, &mut final_destinations, destination);
      }

        source = -1;
        destination = -1;
        count = -1;
      }
    }

    cleanup_seed_locations(&locations, removals, &mut final_destinations);

    locations = final_destinations;
  }

  locations.sort();
  println!("The answer to part 1 is {}", locations[0]);


}

fn cleanup_seed_locations(locations: &Vec<i64>, removals: Vec<usize>, final_destinations: &mut Vec<i64>) {
    for (i, source_location) in locations.iter().enumerate() {
      if !removals.contains(&i) {
        final_destinations.push(*source_location);
      }
    }
}

fn check_seed_location(source: i64, count: i64, source_location: &i64, removals: &mut Vec<usize>, i: usize, final_destinations: &mut Vec<i64>, destination: i64) {
    let limit = source + count;

    if source_location >= &source && source_location < &limit {
    removals.push(i);
    final_destinations.push(destination + (source_location - source));
            }
}

fn build_initial_seed_list(group: &Vec<i64>, locations: &mut Vec<i64>) {
    for location in group {
        locations.push(*location);
      }
}