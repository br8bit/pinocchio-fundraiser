use pinocchio::pubkey::{Pubkey, find_program_address};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Fundraiser {
    pub maker: Pubkey,
    pub mint_to_raise: Pubkey,
    pub target_amount: u64,
    pub raised_amount: u64,
    pub start_time: i64,
    pub duration: u8,
    pub bump: u8,
    is_initialized: bool,
}

impl Fundraiser {
    pub const FUNDRAISER_SEED: &[u8] = b"fundraiser";
    pub const LEN: usize = core::mem::size_of::<Fundraiser>();

    #[inline(always)]
    pub fn key(&self) -> Pubkey {
        find_program_address(&[self.maker.as_ref(), Self::FUNDRAISER_SEED], &crate::ID).0
    }

    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    // #[inline(always)]
    // pub fn set_initialized(&mut self) {
    //     self.is_initialized = true;
    // }
}
