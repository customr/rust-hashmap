use std::time::Instant;
 
mod hash;

pub fn benchmark_my_table(n: usize) {
    
    let mut hash = hash::HashMap::<usize, usize>::new();
    for _ in 0..n {
        let key = rand::random::<usize>();
        if let Some(value) = hash.get_mut(&key) {
            *value += 1;
        } else {
            hash.insert(key, 1);
        }
    }
}

pub fn benchmark_std_table(n: usize) {
    use std::collections::HashMap;

    let mut hash = HashMap::<usize, usize>::new();
    for _ in 0..n {
        let key = rand::random::<usize>();
        if let Some(value) = hash.get_mut(&key) {
            *value += 1;
        } else {
            hash.insert(key, 1);
        }
    }
}

fn main() {
    const N: usize = 100_000;

    let begin = Instant::now();
    benchmark_my_table(N);
    println!("my Table: {}", begin.elapsed().as_secs_f32());

    let begin = Instant::now();
    benchmark_std_table(N);
    println!("std Table: {}", begin.elapsed().as_secs_f32());
}