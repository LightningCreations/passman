pub struct Uuid(u64, u64);

impl Uuid {
    pub const fn new(lo: u64, hi: u64) -> Self {
        Self(lo, hi)
    }
}
