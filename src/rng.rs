/// A very simple randomish number generator, using the [linear
/// congruential](https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use)
/// rng algorithm. The `rand` crate would be preferable, but brings in a lot of
/// baggage and since we're embedded space is at a premium.
pub struct Rng {
    state: u32
}

impl Rng {
    pub fn new(seed: u32) -> Rng {
        Rng {
            state: seed
        }
    }

    /// Generate a randomish number.
    pub fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }

    /// Generate a randomish number in the given range: [low, high).
    /// Inlcudes `low` but not `high`.
    pub fn next_in_range(&mut self, low: i32, high: i32) -> i32 {
        (self.next() % ((high - low) as u32)) as i32 + low
    }
}

    
        
