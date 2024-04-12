use std::time::SystemTime;
use rand::Rng;
fn bubble_sort(array: &Vec<u8>) -> Vec<u8> {
    let mut sorted = array.clone();
    for _ in 0..sorted.len() {
        for j in 0..sorted.len() - 1 {
            if sorted[j] > sorted[j + 1] {
                sorted.swap(j, j + 1);
            }
        }
    }
    return sorted;
}

fn main() {
    let sizes = [
        100, 300, 500, 1000, 1500, 2000, 3000, 4000, 5000, 6000,
        7000, 8000, 9000, 10000, 15000, 20000, 25000, 30000, 4000,
        5000, 60000, 70000, 80000, 90000, 100000
    ];
    let mut vectors: Vec<Vec<u8>> = Vec::with_capacity(25);
    let mut random = rand::thread_rng();
    for size in 0..sizes.len() {
        vectors.push(Vec::new());
        for _ in 0..sizes[size] {
            vectors[size].push(random.gen_range(0..100));
        }
    }

    for (i, vector) in vectors.iter().enumerate() {
        let now = SystemTime::now();
        bubble_sort(vector);
        println!("n: {}, t: {}", vector.len(), now.elapsed().unwrap().as_millis());
    }
}
