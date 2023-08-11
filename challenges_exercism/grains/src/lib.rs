pub fn square(s: u32) -> u64 {
    2^s as u64
}

pub fn total() -> u64 {
    let mut tot: u64 = 0;
    for i in 0..64 {
        tot += 2^i;
    }
    tot
}
