// Given a list of integers, use a vector and return the median (when sorted, the value
// in the middle position) and mode (the value that occurs most often; a hash map will be
// helpful here) of the list.

use std::collections::HashMap;

pub fn median(list_of_integers: &mut Vec<i32>) -> i32 {
    list_of_integers.sort();
    let median = list_of_integers[list_of_integers.len() / 2];
    median
}

pub fn mode(list_of_integers: &Vec<i32>) -> i32 {
    let mut hashmap_of_integers: HashMap<i32, i32> = HashMap::new();

    for value in list_of_integers {
        let count = hashmap_of_integers.entry(*value).or_insert(0);
        *count += 1;
    }

    let mut mode: i32 = list_of_integers[0];
    let mut mode_value: i32 = 0;

    for (key, value) in &hashmap_of_integers {
      if *value > mode_value {
        mode_value = *value;
        mode = *key;
      }
    }

    mode
}
