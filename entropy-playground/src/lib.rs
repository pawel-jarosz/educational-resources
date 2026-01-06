pub struct FileStatistics {
    pub stats: [usize; 256],
    pub length: usize,
}

impl FileStatistics {
    pub fn new() -> FileStatistics {
        FileStatistics {
            stats: [0;256],
            length: 0
        }
    }

    pub fn update(&mut self, buff: &[u8], length: usize) {
        for i in 0..length {
            self.stats[buff[i] as usize] += 1;
            self.length += 1;
        }
    }

    pub fn get_entropy(&self) -> f64 {
        let mut entropy = 0f64;
        for i in 0..self.stats.len() {
            if self.stats[i] == 0 { continue; }
            let probability = self.stats[i] as f64 / self.length as f64;
            entropy -= probability * probability.log2();
        }
        entropy
    }

    pub fn get_used_byte_value(&self) -> usize {
        let mut used_bytes = 0_usize;
        for byte in 0..256_usize {
            if self.stats[byte] == 0 {
                continue;
            }
            used_bytes += 1;
        }
        used_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_statistics() {
        let stats = FileStatistics::new();
        for i in 0..256 {
            assert_eq!(stats.stats[i], 0);
        }
        assert_eq!(stats.length, 0);
    }

    #[test]
    fn test_update() {
        let mut stats = FileStatistics::new();
        let buff = "aabbbaaaaacccccc";
        stats.update(buff.as_bytes(), buff.len());
        assert_eq!(stats.stats['a' as usize], 7);
        assert_eq!(stats.stats['b' as usize], 3);
        assert_eq!(stats.stats['c' as usize], 6);
        assert_eq!(stats.get_used_byte_value(), 3);
        let complementary = "bbbbbcaccabs";
        stats.update(complementary.as_bytes(), complementary.len());
        assert_eq!(stats.stats['a' as usize], 9);
        assert_eq!(stats.stats['b' as usize], 9);
        assert_eq!(stats.stats['c' as usize], 9);
        assert_eq!(stats.stats['s' as usize], 1);
        assert_eq!(stats.get_used_byte_value(), 4);
    }

    #[test]
    fn test_get_entropy() {
        let mut stats = FileStatistics::new();
        let buff = "aabbbaaaaacccccc";
        stats.update(buff.as_bytes(), buff.len());
        let entropy = stats.get_entropy();
        assert!((entropy - 1.505).abs() < 0.001);
    }
}