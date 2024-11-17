#![warn(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic, clippy::perf)]
#![allow(clippy::cargo_common_metadata, clippy::nursery)]
#![forbid(unsafe_code)]

/**
* rSentenceHash
* Copyright (C) 2024  TheTrueColonel
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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

    (0..usable_threads)
        .into_par_iter()
        .for_each(|_| {
            // Allocate buffers
            let mut str_iteration_buffer: Vec<u8> = Vec::with_capacity(16); // Can hold all hex values up to u64::MAX
            let mut sentence_buffer: Vec<u8> = Vec::with_capacity(64);

            let mut hasher = Sha256::new();

            while !MATCHED.load(Ordering::Acquire) {
                let current_iteration = ITERATION.fetch_add(1, Ordering::AcqRel);

                str_iteration_buffer.clear();
                #[allow(clippy::cast_possible_truncation)]
                unsigned_num_to_hex(current_iteration as usize, &mut str_iteration_buffer);

                let str_iteration_end = &str_iteration_buffer[str_iteration_buffer.len() - 9..];

                sentence_buffer.clear();
                sentence_buffer.extend(BASE_SENTENCE.as_bytes());
                sentence_buffer.extend(str_iteration_end);

                hasher.update(&sentence_buffer);

                let sentence_hash = hasher.finalize_reset();
                let sentence_hex = base16ct::upper::encode_string(&sentence_hash);
                let hash_end = &sentence_hex[sentence_hex.len() - 9..];

                if str_iteration_end == hash_end.as_bytes() {
                    let mut sw = sw.lock().unwrap();

                    MATCHED.store(true, Ordering::Release);

                    sw.stop().unwrap();
                    println!("Finished in: {:?} | Iteration Count: {} | Threads Used: {}", sw.elapsed(), current_iteration, usable_threads);
                    println!("Sentence: {} \nFull Sentence Hash: {sentence_hex} \nHash End: {hash_end} \nIteration Hash End: {hash_end}", String::from_utf8_lossy(&sentence_buffer));

                    break;
                }

                if current_iteration % LOG_INTERVAL == 0 {
                    println!("{current_iteration} | Iter Str: {} | Hash End Str: {hash_end}", String::from_utf8_lossy(str_iteration_end));
                }
            }
        });
}