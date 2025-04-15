#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Contributor {
    pub is_initialized: bool,
    pub amount: u64,
}

impl Contributor {
    pub const CONTRIBUTOR_SEED: &'static [u8] = b"contributor";
    pub const LEN: usize = core::mem::size_of::<Self>();

    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    #[inline(always)]
    pub fn initialize(&mut self, amount: u64) {
        self.is_initialized = true;
        self.amount = amount;
    }

    #[inline(always)]
    pub fn update(&mut self, amount: u64) {
        self.amount = amount;
    }
}
