use rand::RngCore;

#[inline]
pub fn gen_random() -> u64 {
    embassy_rp::clocks::RoscRng.next_u64()
}
