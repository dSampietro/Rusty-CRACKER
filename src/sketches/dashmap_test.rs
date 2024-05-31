use dashmap::DashMap;
use rayon::prelude::*;

fn main() {
    // Create a sample DashMap
    let map = DashMap::new();
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    map.insert(4, "four");
    map.insert(5, "five");

    // Use par_iter to iterate over the DashMap in parallel and filter_map
    let result: DashMap<_, _> = map.par_iter()
        .filter_map(|entry| {
            let (&key, &value) = entry.pair();
            if key % 2 == 0 {
                // Keep even keys and convert the value to uppercase
                Some((key, value.to_uppercase()))
            } else {
                // Filter out odd keys
                None
            }
        })
        .collect();

    // Print the result
    result.iter().for_each(|entry| {
        let (key, value) = entry.pair();
        println!("Key: {}, Value: {}", key, value);
    });
}