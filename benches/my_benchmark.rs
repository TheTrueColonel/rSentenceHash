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

use criterion::{criterion_group, criterion_main, Criterion};
use lazy_static::lazy_static;
use rSentenceHash::unsigned_num_to_hex;

lazy_static! {
    static ref BUFFER: String = String::with_capacity(16);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("num_to_hex", |b| { 
        b.iter(|| {
            let mut buffer = BUFFER.clone();
            unsigned_num_to_hex(48_405_995_369, &mut buffer)
        }) 
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);