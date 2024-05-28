use std::time::Duration;

use hex_literal::hex;
use rand::Rng;
use sha2::{Digest, Sha256};
fn main() {
    let mut rng = rand::thread_rng();
    let mut total_duration = Duration::new(0, 0);

    for _ in 0..100000 {
        let mut hasher = Sha256::new();
        let mut data = [0u8; 2048];
        rng.fill(&mut data);

        let start = std::time::Instant::now();
        hasher.update(&data);
        let _hash256 = hasher.finalize();
        total_duration += start.elapsed();
    }

    let average_duration = total_duration / 100000;
    println!(
        "Average SHA-256 hash time on 1024 random bytes: {:?}",
        average_duration
    );
}
