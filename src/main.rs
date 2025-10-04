use bit_map::BitMap;
use bit_map::BitMap64;
use std::collections::HashMap;
use std::time::Instant;
fn main() {
    let n = 10_000;

    // === BitMap<u8> ===
    let mut bitmap = BitMap::with_capacity(8);

    let start = Instant::now();
    for i in 0..n {
        bitmap.insert(i, i * 10);
    }
    let insert_time = start.elapsed();
    println!("BitMap: Inserted {} elements in {:?}", n, insert_time);

    let start = Instant::now();
    for i in 0..n {
        assert_eq!(bitmap.get(&i), Some(&(i * 10)));
    }
    let lookup_time = start.elapsed();
    println!("BitkMap: Looked up {} elements in {:?}", n, lookup_time);

    let start = Instant::now();
    for i in 0..n {
        assert_eq!(bitmap.delete(&i), Some(i * 10));
    }
    let delete_time = start.elapsed();
    println!("BitMap: Deleted {} elements in {:?}", n, delete_time);

    // === BitMap<u64> ===
    let mut bitmap64 = BitMap64::with_capacity(8);

    let start = Instant::now();
    for i in 0..n {
        bitmap64.insert(i, i * 10);
    }
    let insert_time = start.elapsed();
    println!("BitMap64: Inserted {} elements in {:?}", n, insert_time);

    let start = Instant::now();
    for i in 0..n {
        assert_eq!(bitmap64.get(&i), Some(&(i * 10)));
    }
    let lookup_time = start.elapsed();
    println!("BitMap64: Looked up {} elements in {:?}", n, lookup_time);

    let start = Instant::now();
    for i in 0..n {
        assert_eq!(bitmap64.delete(&i), Some(i * 10));
    }
    let delete_time = start.elapsed();
    println!("BitMap64: Deleted {} elements in {:?}", n, delete_time);

    // === Std HashMap ===
    let mut hashmap = HashMap::with_capacity(n);

    let start = Instant::now();
    for i in 0..n {
        hashmap.insert(i, i * 10);
    }
    let insert_time = start.elapsed();
    println!("Std HashMap: Inserted {} elements in {:?}", n, insert_time);

    let start = Instant::now();
    for i in 0..n {
        assert_eq!(hashmap.get(&i), Some(&(i * 10)));
    }
    let lookup_time = start.elapsed();
    println!("Std HashMap: Looked up {} elements in {:?}", n, lookup_time);

    let start = Instant::now();
    for i in 0..n {
        assert_eq!(hashmap.remove(&i), Some(i * 10));
    }
    let delete_time = start.elapsed();
    println!("Std HashMap: Deleted {} elements in {:?}", n, delete_time);
}
