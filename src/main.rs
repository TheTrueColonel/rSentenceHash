#![warn(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic, clippy::perf)]
#![allow(clippy::cargo_common_metadata, clippy::nursery)]
#![forbid(unsafe_code)]

use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread::available_parallelism;
use color_eyre::eyre::Result;
use libsw::Sw;
use sha2::{Digest, Sha256};
use rayon::prelude::*;
use rSentenceHash::{unsigned_num_to_hex_tail};

const BASE_SENTENCE: &str = "The last 9 of this fucking sentence's SHA256 is ";

static ITERATION: AtomicU64 = AtomicU64::new(0);
static MATCHED: AtomicBool = AtomicBool::new(false);
static LOG_INTERVAL: u64 = 10_000_000;
static BATCH_SIZE: u64 = 1_000_000;

fn main() -> Result<()> {
    color_eyre::install()?;
    
    let thread_count = u16::try_from(available_parallelism()?.get()).map_or(u16::MAX, |n| n);
    #[allow(clippy::cast_possible_truncation)]
    let usable_threads = ((u32::from(thread_count) * 3) / 4) as u16; // Can't possibly overflow u16 here

    compute_hashes(usable_threads);
    
    Ok(())
}

fn compute_hashes(usable_threads: u16) {
    let sw = Arc::new(Mutex::new(Sw::new_started()));

    let mut hasher = Sha256::new();

    hasher.update(BASE_SENTENCE.as_bytes());

    let hasher = hasher;

    (0..usable_threads)
        .into_par_iter()
        .for_each(|_| {
            // Allocate buffers
            let mut str_iteration_buffer = [0u8; 10];
            let mut tail_hex = [0u8; 10];

            while !MATCHED.load(Ordering::Acquire) {
                let mut current_iteration = ITERATION.fetch_add(BATCH_SIZE, Ordering::AcqRel);

                for _ in 0..BATCH_SIZE {
                    if MATCHED.load(Ordering::Relaxed) { break; }

                    unsigned_num_to_hex_tail(current_iteration, &mut str_iteration_buffer);

                    let str_iteration_end = &str_iteration_buffer[1..];

                    let mut hasher_clone = hasher.clone();

                    hasher_clone.update(str_iteration_end);

                    let hash_output = hasher_clone.finalize();

                    // Compare bytes directory to determine if matched
                    let n_bytes = current_iteration.to_be_bytes();
                    let found = (hash_output[27] & 0x0F) == (n_bytes[3] & 0x0F) &&
                        hash_output[28..32] == n_bytes[4..8];

                    if found {
                        let mut sw = sw.lock().unwrap();

                        sw.stop().unwrap();

                        let final_sentence = format!("{}{}", BASE_SENTENCE, String::from_utf8_lossy(str_iteration_end));

                        MATCHED.store(true, Ordering::Release);

                        base16ct::upper::encode(&hash_output[27..], &mut tail_hex).unwrap();
                        let hash_end = &tail_hex[1..];

                        let mut sentence_hex_buffer = [0u8; 64];
                        base16ct::upper::encode(&hash_output, &mut sentence_hex_buffer).unwrap();

                        println!("Finished in: {:?} | Iteration Count: {} | Threads Used: {}", sw.elapsed(), current_iteration, usable_threads);
                        println!("Sentence: {final_sentence} \nFull Sentence Hash: {} \nHash End: {} \nIteration Hash End: {}", String::from_utf8_lossy(&sentence_hex_buffer), String::from_utf8_lossy(hash_end), String::from_utf8_lossy(str_iteration_end));

                        break;
                    }

                    if current_iteration.is_multiple_of(LOG_INTERVAL) {
                        base16ct::upper::encode(&hash_output[27..], &mut tail_hex).unwrap();
                        let hash_end = &tail_hex[1..];

                        println!("{current_iteration} | Iter Str: {} | Hash End Str: {}", String::from_utf8_lossy(str_iteration_end), String::from_utf8_lossy(hash_end));
                    }

                    current_iteration += 1;
                }
            }
        });
}