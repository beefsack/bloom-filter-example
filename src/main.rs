extern crate rand;

use rand::{Rng, SeedableRng, XorShiftRng};
use std::env;

const SIZE: usize = 1024;

fn main() {
    // Get the number of hash functions to use as an argument
    let num_filters: usize = env::args()
        .skip(1)
        .next()
        .map(|a| a.parse().expect("filters argument must be a number"))
        .unwrap_or(HASH_FNS.len());

    // We use a seedable RNG so we can reliably repeat the test
    let mut rng = XorShiftRng::from_seed([123, 456, 789, 0]);

    // For simplicity, the bloom filter is represented as an array of booleans
    let mut filter: [bool; SIZE] = [false; SIZE];

    // For the test, input is unique values larger than all the hash function moduli
    let mut input: Vec<usize> = (900000..1000000).take(SIZE).collect();
    rng.shuffle(&mut input);

    // Take some amount of the input and count false positives while inserting into the bloom filter
    let mut false_positives: usize = 0;
    for i in input.into_iter() {
        if may_contain(&filter, i, num_filters) {
            false_positives += 1;
        }
        println!("{}", false_positives);
        insert(&mut filter, i, num_filters);
    }
}

// Insert a value into the bloom filter using a specified amount of hash functions
fn insert(filter: &mut [bool; SIZE], value: usize, num_filters: usize) {
    for f in HASH_FNS.iter().take(num_filters) {
        filter[f(value) % SIZE] = true;
    }
}

// Check if the bloom filter may contain a value usign a specified amount of hash functions
fn may_contain(filter: &[bool; SIZE], value: usize, num_filters: usize) -> bool {
    HASH_FNS
        .iter()
        .take(num_filters)
        .find(|f| !filter[f(value) % SIZE])
        .is_none()
}

// The actual hash functions are just using modulus for simplicity sake
fn hash_a(value: usize) -> usize {
    value % 1674
}
fn hash_b(value: usize) -> usize {
    value % 3496
}
fn hash_c(value: usize) -> usize {
    value % 5998
}
type HashFn = fn(usize) -> usize;
static HASH_FNS: [HashFn; 3] = [hash_a, hash_b, hash_c];