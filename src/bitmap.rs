use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Slot states:
// 00 = Empty
// 01 = Occupied
// 10 = Deleted
fn set_state(bitmap: &mut [u8], index: usize, state: u8) {
    let byte_index = index / 4; // 4 slots per byte
    let bit_offset = (index % 4) * 2; // 2 bits per slot
    let mask = 0b11 << bit_offset;
    bitmap[byte_index] = (bitmap[byte_index] & !mask) | ((state & 0b11) << bit_offset);
}

fn get_state(bitmap: &[u8], index: usize) -> u8 {
    let byte_index = index / 4;
    let bit_offset = (index % 4) * 2;
    (bitmap[byte_index] >> bit_offset) & 0b11
}

pub struct BitMap<K, V> {
    bitmap: Vec<u8>,              // 2 bits per slot
    buckets: Vec<Option<(K, V)>>, // slot storage
    capacity: usize,
    size: usize,
}

impl<K: Hash + Eq, V> BitMap<K, V> {
    // Create new map with default capacity (8 slots)
    pub fn new() -> Self {
        Self::with_capacity(8)
    }

    // Create new map with user capacity, rounded up to next power of two
    pub fn with_capacity(c: usize) -> Self {
        let capacity = c.next_power_of_two().max(8);
        let bitmap_size = (capacity * 2 + 7) / 8;
        Self {
            bitmap: vec![0; bitmap_size],
            buckets: (0..capacity).map(|_| None).collect(),
            capacity,
            size: 0,
        }
    }

    // Hash key to slot index
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    // Insert (key, value). Updates if key already exists.
    pub fn insert(&mut self, key: K, val: V) {
        let mut index = self.hash(&key);

        loop {
            match get_state(&self.bitmap, index) {
                0 => {
                    // Empty
                    self.buckets[index] = Some((key, val));
                    set_state(&mut self.bitmap, index, 1);
                    self.size += 1;
                    break;
                }
                2 => {
                    // Deleted
                    self.buckets[index] = Some((key, val));
                    set_state(&mut self.bitmap, index, 1);
                    self.size += 1;
                    break;
                }
                1 => {
                    // Occupied
                    if let Some((ref k, ref mut v)) = self.buckets[index] {
                        if *k == key {
                            *v = val;
                            return;
                        }
                    }
                    index = (index + 1) % self.capacity;
                }
                _ => unreachable!(),
            }
        }

        // Resize if load factor too high >= 0.75
        if self.size * 4 >= self.capacity * 3 {
            self.resize();
        }
    }

    // Lookup key
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut index = self.hash(key);

        loop {
            match get_state(&self.bitmap, index) {
                0 => return None,
                1 => {
                    if let Some((ref k, ref v)) = self.buckets[index] {
                        if k == key {
                            return Some(v);
                        }
                    }
                    index = (index + 1) % self.capacity;
                }
                2 => {
                    index = (index + 1) % self.capacity;
                }
                _ => unreachable!(),
            }
        }
    }

    // Delete key and return value if found
    pub fn delete(&mut self, key: &K) -> Option<V> {
        let mut index = self.hash(key);

        loop {
            match get_state(&self.bitmap, index) {
                0 => return None,
                1 => {
                    if let Some((ref k, _)) = self.buckets[index] {
                        if k == key {
                            let entry = self.buckets[index].take();
                            set_state(&mut self.bitmap, index, 2);
                            self.size -= 1;
                            return entry.map(|(_, v)| v);
                        }
                    }
                    index = (index + 1) % self.capacity;
                }
                2 => {
                    index = (index + 1) % self.capacity;
                }
                _ => unreachable!(),
            }
        }
    }

    // Resize to 2x capacity
    fn resize(&mut self) {
        let new_capacity = self.capacity * 2;
        let new_bitmap_size = (new_capacity * 2 + 7) / 8;
        let mut new_map = BitMap::<K, V> {
            bitmap: vec![0; new_bitmap_size],
            buckets: (0..new_capacity).map(|_| None).collect(),
            capacity: new_capacity,
            size: 0,
        };

        for slot in self.buckets.drain(..) {
            if let Some((k, v)) = slot {
                new_map.insert(k, v);
            }
        }

        *self = new_map;
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
