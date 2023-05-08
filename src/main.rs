use itertools::iproduct;
use md5::{Digest, Md5};
use rayon::prelude::*;
use std::env;
use std::time::Instant;

fn md5_hash(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn check_ip(ip: &str, source_hash: &str) -> Option<String> {
    let ip_hash = md5_hash(ip);
    if ip_hash == source_hash {
        Some(ip.to_string())
    } else {
        None
    }
}

fn find_matching_ip(source_hash: &str) -> Option<String> {
    iproduct!(0..=255u8, 0..=255u8, 0..=255u8, 0..=255u8)
        .par_bridge()
        .filter_map(|(i, j, k, l)| {
            let ip = format!("{}.{}.{}.{}", i, j, k, l);
            check_ip(&ip, source_hash)
        })
        .find_any(|_| true)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <hash_1> <hash_2> ... <hash_n>", args[0]);
        std::process::exit(1);
    }

    let input_hashes = &args[1..];
    for (idx, source_hash) in input_hashes.iter().enumerate() {
        let start_time = Instant::now();
        match find_matching_ip(source_hash) {
            Some(ip) => {
                let elapsed = start_time.elapsed();
                println!(
                    "Found a match for source IP hash {}: {} at IP: {} (elapsed time: {:?})",
                    idx + 1,
                    source_hash,
                    ip,
                    elapsed
                )
            }
            None => {
                let elapsed = start_time.elapsed();
                println!(
                    "No match found for source IP hash {}: {} (elapsed time: {:?})",
                    idx + 1,
                    source_hash,
                    elapsed
                )
            }
        }
    }
}
