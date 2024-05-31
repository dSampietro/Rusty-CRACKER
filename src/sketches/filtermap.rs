use std::collections::HashMap;

fn main() {
    // Example HashMap
    let mut map: HashMap<u8, Vec<u8>> = HashMap::new();
    map.insert(1, vec![3, 2, 1]);
    map.insert(2, vec![6, 5, 4]);
    map.insert(3, vec![]); // Empty vector
    map.insert(4, vec![9, 7, 8]);

    // Map to find the minimum of each Vec<u8>
    let min_map: HashMap<u8, u8> = map.into_iter()
        .filter_map(|(key, values)| {
            values.into_iter().min().map(|min| (key, min))
        })
        .collect();

    // Print the resulting HashMap
    for (key, min) in &min_map {
        println!("Node {}: Min neighbor {}", key, min);
    }
}