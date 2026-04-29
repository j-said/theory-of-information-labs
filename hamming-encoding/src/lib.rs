pub struct HammingCodec {
    total_bits: usize,
    parity_bits: usize,
}

impl HammingCodec {
    pub fn new(data_bits: usize) -> Self {
        let mut r = 0;
        while (1 << r) < (data_bits + r + 1) {
            r += 1;
        }
        Self {
            total_bits: data_bits + r,
            parity_bits: r,
        }
    }

    pub fn encode(&self, data: u32) -> u32 {
        let mut encoded = 0;
        let mut data_idx = 0;

        for i in 1..=self.total_bits {
            if !self.is_parity_pos(i) {
                encoded |= ((data >> data_idx) & 1) << (i - 1);
                data_idx += 1;
            }
        }

        for p in 0..self.parity_bits {
            let parity_pos = 1 << p;
            let mut parity_val = 0;
            for i in 1..=self.total_bits {
                if (i & parity_pos) != 0 {
                    parity_val ^= (encoded >> (i - 1)) & 1;
                }
            }
            encoded |= parity_val << (parity_pos - 1);
        }
        encoded
    }

    pub fn decode(&self, mut encoded: u32) -> u32 {
        let mut syndrome = 0;
        for p in 0..self.parity_bits {
            let parity_pos = 1 << p;
            let mut parity_val = 0;
            for i in 1..=self.total_bits {
                if (i & parity_pos) != 0 {
                    parity_val ^= (encoded >> (i - 1)) & 1;
                }
            }
            if parity_val != 0 {
                syndrome |= parity_pos;
            }
        }

        if syndrome > 0 && syndrome <= self.total_bits {
            encoded ^= 1 << (syndrome - 1);
        }

        let mut decoded = 0;
        let mut data_idx = 0;
        for i in 1..=self.total_bits {
            if !self.is_parity_pos(i) {
                decoded |= ((encoded >> (i - 1)) & 1) << data_idx;
                data_idx += 1;
            }
        }
        decoded
    }

    #[inline]
    fn is_parity_pos(&self, pos: usize) -> bool {
        (pos & (pos - 1)) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_8bit_clean() {
        let codec = HammingCodec::new(8);
        for origin in 0..=255 {
            let encoded = codec.encode(origin);
            let decoded = codec.decode(encoded);
            assert_eq!(origin, decoded, "Failed clean on 8-bit origin: {}", origin);
        }
    }

    #[test]
    fn test_generic_8bit_single_error() {
        let codec = HammingCodec::new(8);
        for origin in 0..=255 {
            let encoded = codec.encode(origin);
            for error_bit in 0..codec.total_bits {
                let noisy = encoded ^ (1 << error_bit);
                let decoded = codec.decode(noisy);
                assert_eq!(
                    origin, decoded,
                    "Correction failed at bit {} for origin {}",
                    error_bit, origin
                );
            }
        }
    }

    #[test]
    fn test_generic_16bit_clean_and_noisy() {
        let codec = HammingCodec::new(16);
        
        // Test edge cases and scattered samples for speed
        let samples = [0, 1, 42, 255, 1024, 65535, 43981];
        
        for origin in samples {
            let encoded = codec.encode(origin);
            
            // Clean
            assert_eq!(origin, codec.decode(encoded));
            
            // Single error
            for error_bit in 0..codec.total_bits {
                let noisy = encoded ^ (1 << error_bit);
                assert_eq!(origin, codec.decode(noisy));
            }
        }
    }
}