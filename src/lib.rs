#![allow(non_snake_case)]

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

#[inline]
pub fn unsigned_num_to_hex(n: usize, buffer: &mut String) {
    if n == 0 {
        buffer.push('0');
    } else {
        let mut n = n;

        while n > 0 {
            #[allow(clippy::cast_possible_truncation)]
            let digit = (n & 0xF) as u8;

            n >>= 4;

            let hex_char = match digit {
                0..=9 => (digit + b'0') as char,
                10..=15 => (digit - 10 + b'A') as char,
                _ => unreachable!(),
            };

            buffer.push(hex_char);
        }
    }

    // Ensure buffer is full before returning
    while buffer.len() < buffer.capacity() {
        buffer.push('0');
    }

    *buffer = buffer.chars().rev().collect::<String>();
}