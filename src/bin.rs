use rand::prelude::*;
use rust_batched::batched::*;

fn this_fn(s: &String) {
    println!("OK in main this_fn {}", s);
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums: Vec<String> = (1..1010000).map(|d| d.to_string()).collect();
    nums.shuffle(&mut rng);
    with_crossbeam(nums.into_iter(), 6, this_fn)
}