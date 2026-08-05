//! Piccolo PRNG deterministico (SplitMix64) seedabile.
//!
//! Serve per pescare e mescolare le carte: seedato da entropia di sistema in
//! produzione, ma seedabile a mano nei test per esiti riproducibili.

pub struct Rng {
    state: u64,
}

impl Rng {
    /// Seed esplicito, per esiti riproducibili nei test.
    #[allow(dead_code)]
    pub fn with_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Seedato dall'entropia del sistema; se non disponibile, ripiega sul tempo.
    pub fn from_entropy() -> Self {
        let mut buf = [0u8; 8];
        let seed = if getrandom::fill(&mut buf).is_ok() {
            u64::from_le_bytes(buf)
        } else {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0x9E3779B97F4A7C15)
        };
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        // SplitMix64.
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }

    /// Intero uniforme in `0..n` (n > 0).
    pub fn below(&mut self, n: usize) -> usize {
        if n == 0 {
            return 0;
        }
        (self.next_u64() % n as u64) as usize
    }

    pub fn bool_with(&mut self, chance: f32) -> bool {
        (self.next_u64() as f64 / u64::MAX as f64) < chance as f64
    }

    /// Mescola in-place (Fisher-Yates).
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        for i in (1..items.len()).rev() {
            let j = self.below(i + 1);
            items.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_same_sequence() {
        let mut a = Rng::with_seed(42);
        let mut b = Rng::with_seed(42);
        for _ in 0..100 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn below_stays_in_range() {
        let mut r = Rng::with_seed(7);
        for _ in 0..1000 {
            assert!(r.below(10) < 10);
        }
        assert_eq!(r.below(0), 0);
    }

    #[test]
    fn shuffle_is_a_permutation() {
        let mut r = Rng::with_seed(123);
        let mut v: Vec<usize> = (0..50).collect();
        r.shuffle(&mut v);
        let mut sorted = v.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..50).collect::<Vec<_>>());
        // Con questo seed non deve restare identico all'ordine iniziale.
        assert_ne!(v, (0..50).collect::<Vec<_>>());
    }

    #[test]
    fn bool_with_extremes() {
        let mut r = Rng::with_seed(1);
        assert!(!r.bool_with(0.0));
        assert!(r.bool_with(1.0));
    }
}
