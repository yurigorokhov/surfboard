use rand_core::CryptoRng;
use rand_core::RngCore;

#[derive(Clone)]
pub struct RngWrapper;

impl RngWrapper {
    pub fn new() -> Self {
        Self {}
    }
}

impl RngCore for RngWrapper {
    fn next_u32(&mut self) -> u32 {
        1337
    }

    fn next_u64(&mut self) -> u64 {
        u32_pair_to_u64(self.next_u32(), self.next_u32())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for value in dest.iter_mut() {
            let [random_value, _, _, _] = self.next_u32().to_ne_bytes();
            *value = random_value;
        }
    }
}

impl CryptoRng for RngWrapper {}

/// Join a pair of `u32` into a `u64`
#[allow(
    clippy::many_single_char_names,
    clippy::min_ident_chars,
    reason = "This is still readable"
)]
fn u32_pair_to_u64(first: u32, second: u32) -> u64 {
    let [a, b, c, d] = first.to_ne_bytes();
    let [e, f, g, h] = second.to_ne_bytes();
    u64::from_ne_bytes([a, b, c, d, e, f, g, h])
}
