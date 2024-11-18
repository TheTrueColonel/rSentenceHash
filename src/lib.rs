#![allow(non_snake_case)]

/// Encodes `input_number` into `buffer` in hexadecimal format
/// 
/// (No length checking as this is specialized. Take the small performance hit for checks if using more generally.)
/// 
/// # Panics
/// If size of number in bytes and buffer lengths are misaligned
#[inline]
pub fn unsigned_num_to_hex(input_number: usize, buffer: &mut [u8]) {
    let num_bytes = input_number.to_be_bytes();
    
    for (byte, dst) in num_bytes.iter().zip(buffer.chunks_exact_mut(2)) {
        dst[0] = encode_nibble(byte >> 4);
        dst[1] = encode_nibble(byte & 0xF);
    }
}

#[inline(always)]
fn encode_nibble(num: u8) -> u8 {
    let mut n = num as i16 + 0x30;

    n += ((0x39i16 - n) >> 8) & (0x41i16 - 0x3a);
    n as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn num_to_hex_min() {
        let mut buffer = vec![0u8; 16];
        
        unsigned_num_to_hex(0, &mut buffer);
        
        assert_eq!(String::from_utf8(buffer).unwrap(), "0000000000000000");
    }
    
    #[test]
    fn num_to_hex_max() {
        let mut buffer = vec![0u8; 16];

        unsigned_num_to_hex(usize::MAX, &mut buffer);

        assert_eq!(String::from_utf8(buffer).unwrap(), "FFFFFFFFFFFFFFFF");
    }
    
    #[test]
    fn hex_to_num_random_full_buffer() {
        let mut buffer = vec![0u8; 16];
        let inputs: Vec<usize> = vec![696945551951787582, 14670462735960287247, 16883653885393442501, 18041941842666528556, 5835101380978880591,
                                      984250785791736972, 18131007051776252294, 14105752990456256504, 7912357114648305790, 7027766012445430219];
        let results: Vec<&str> = vec!["09AC0C4973DFF63E", "CB97F4218384A80F", "EA4EC9C15D8516C5", "FA61DA83FE651F2C", "50FA71CA51A3304F", 
                                      "0DA8C2DFF903FC8C", "FB9E46D9F539D986", "C3C1B39BFD8C97F8", "6DCE56DFD2A5A07E", "6187A3F4205F91CB"];
        
        for i in 0..inputs.len() {
            unsigned_num_to_hex(inputs[i], &mut buffer);
            assert_eq!(String::from_utf8_lossy(&buffer), results[i]);
        }
    }

    #[test]
    fn hex_to_num_random_non_full_buffer() {
        let mut buffer = vec![0u8; 16];
        let inputs: Vec<usize> = vec![2789444564, 695265371, 283150, 1916511949, 564194367,
                                      662333843, 22, 741749512, 24718396, 3291403215];
        let results: Vec<&str> = vec!["00000000A6438BD4", "000000002970E85B", "000000000004520E", "00000000723BA6CD", "0000000021A0EC3F",
                                      "00000000277A6993", "0000000000000016", "000000002C363308", "0000000001792C3C", "00000000C42ED3CF"];

        for i in 0..inputs.len() {
            unsigned_num_to_hex(inputs[i], &mut buffer);
            assert_eq!(String::from_utf8_lossy(&buffer), results[i]);
        }
    }
}