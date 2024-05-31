use dashmap::DashMap;
use rayon::prelude::*;

fn main() {
    // Create and populate the original DashMap
    let original_map: DashMap<&str, Vec<i32>> = DashMap::new();
    original_map.insert("key1", vec![1, 2, 3]);
    original_map.insert("key2", vec![4, 5, 6]);
    original_map.insert("key3", vec![7, 8, 9]);

    // Create a new DashMap to store the minimum values
    let min_values_map: DashMap<&str, i32> = DashMap::new();

    // Collect entries into a vector
    let entries: Vec<_> = original_map.iter().collect();

    // Use Rayon to find the minimum values in parallel
    entries.par_iter().for_each(|entry| {
        let (key, vec) = entry.pair();
        if let Some(&min_value) = vec.iter().min() {
            min_values_map.insert(key, min_value);
        }
    });

    // Print the new DashMap
    min_values_map.iter().for_each(|entry| {
        println!("Key: {}, Min Value: {}", entry.key(), entry.value());
    });
}
