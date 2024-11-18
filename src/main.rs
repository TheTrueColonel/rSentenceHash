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
use rSentenceHash::unsigned_num_to_hex;

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
    
    (0..usable_threads)
        .into_par_iter()
        .for_each(|_| {
            // Allocate buffers
            let mut str_iteration_buffer: Vec<u8> = vec![0u8; 16];
            let mut hasher_finalize_buffer = vec![0u8; 32];
            let mut sentence_hex_buffer = vec![0u8; 64];

            while !MATCHED.load(Ordering::Acquire) {
                let mut current_iteration = ITERATION.fetch_add(BATCH_SIZE, Ordering::AcqRel);

                for _ in 0..BATCH_SIZE {
                    #[allow(clippy::cast_possible_truncation)]
                    unsigned_num_to_hex(current_iteration as usize, &mut str_iteration_buffer);

                    let str_iteration_end = &str_iteration_buffer[str_iteration_buffer.len() - 9..];

                    let mut hasher_clone = hasher.clone();

                    hasher_clone.update(str_iteration_end);

                    hasher_clone.finalize_into(hasher_finalize_buffer.as_mut_slice().into());

                    base16ct::upper::encode(&hasher_finalize_buffer, &mut sentence_hex_buffer).unwrap();

                    let hash_end = &sentence_hex_buffer[sentence_hex_buffer.len() - 9..];

                    if str_iteration_end == hash_end {
                        let mut sw = sw.lock().unwrap();
                        let final_sentence = format!("{}{}", BASE_SENTENCE, String::from_utf8_lossy(str_iteration_end));

                        MATCHED.store(true, Ordering::Release);

                        sw.stop().unwrap();
                        println!("Finished in: {:?} | Iteration Count: {} | Threads Used: {}", sw.elapsed(), current_iteration, usable_threads);
                        println!("Sentence: {final_sentence} \nFull Sentence Hash: {} \nHash End: {} \nIteration Hash End: {}", String::from_utf8_lossy(&sentence_hex_buffer), String::from_utf8_lossy(hash_end), String::from_utf8_lossy(str_iteration_end));

                        break;
                    }

                    if current_iteration % LOG_INTERVAL == 0 {
                        println!("{current_iteration} | Iter Str: {} | Hash End Str: {}", String::from_utf8_lossy(str_iteration_end), String::from_utf8_lossy(hash_end));
                    }

                    current_iteration += 1;
                }
            }
        });
}