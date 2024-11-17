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
pub fn unsigned_num_to_hex(n: usize, buffer: &mut Vec<u8>) {
    if n == 0 {
        buffer.push(b'0');
    } else {
        let mut n = n;

        while n > 0 {
            #[allow(clippy::cast_possible_truncation)]
            let digit = (n & 0xF) as u8;

            n >>= 4;

            let hex_char = match digit {
                0..=9 => digit + b'0',
                10..=15 => digit - 10 + b'A',
                _ => unreachable!(),
            };

            buffer.push(hex_char);
        }
    }

    // Ensure buffer is full before returning
    while buffer.len() < buffer.capacity() {
        buffer.push(b'0');
    }

    buffer.reverse();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn num_to_hex_min() {
        let mut buffer = Vec::with_capacity(16);
        
        unsigned_num_to_hex(0, &mut buffer);
        
        assert_eq!(String::from_utf8(buffer).unwrap(), "0000000000000000");
    }
    
    #[test]
    fn num_to_hex_max() {
        let mut buffer = Vec::with_capacity(16);

        unsigned_num_to_hex(usize::MAX, &mut buffer);

        assert_eq!(String::from_utf8(buffer).unwrap(), "FFFFFFFFFFFFFFFF");
    }
    
    #[test]
    fn hex_to_num_random_full_buffer() {
        let mut buffer = Vec::with_capacity(16);
        let inputs: Vec<usize> = vec![696945551951787582, 14670462735960287247, 16883653885393442501, 18041941842666528556, 5835101380978880591,
                                      984250785791736972, 18131007051776252294, 14105752990456256504, 7912357114648305790, 7027766012445430219];
        let results: Vec<&str> = vec!["09AC0C4973DFF63E", "CB97F4218384A80F", "EA4EC9C15D8516C5", "FA61DA83FE651F2C", "50FA71CA51A3304F", 
                                      "0DA8C2DFF903FC8C", "FB9E46D9F539D986", "C3C1B39BFD8C97F8", "6DCE56DFD2A5A07E", "6187A3F4205F91CB"];
        
        for i in 0..inputs.len() {
            buffer.clear();
            unsigned_num_to_hex(inputs[i], &mut buffer);
            assert_eq!(String::from_utf8_lossy(&buffer), results[i]);
        }
    }

    #[test]
    fn hex_to_num_random_non_full_buffer() {
        let mut buffer = Vec::with_capacity(16);
        let inputs: Vec<usize> = vec![2789444564, 695265371, 283150, 1916511949, 564194367,
                                      662333843, 22, 741749512, 24718396, 3291403215];
        let results: Vec<&str> = vec!["00000000A6438BD4", "000000002970E85B", "000000000004520E", "00000000723BA6CD", "0000000021A0EC3F",
                                      "00000000277A6993", "0000000000000016", "000000002C363308", "0000000001792C3C", "00000000C42ED3CF"];

        for i in 0..inputs.len() {
            buffer.clear();
            unsigned_num_to_hex(inputs[i], &mut buffer);
            assert_eq!(String::from_utf8_lossy(&buffer), results[i]);
        }
    }
}