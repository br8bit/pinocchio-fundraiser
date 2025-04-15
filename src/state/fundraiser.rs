use pinocchio::pubkey::{Pubkey, find_program_address};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Fundraiser {
    is_initialized: bool,
    pub maker: Pubkey,
    pub mint_to_raise: Pubkey,
    pub target_amount: u64,
    pub raised_amount: u64,
    pub start_time: i64,
    pub duration: u8,
    pub bump: u8,
}

impl Fundraiser {
    pub const FUNDRAISER_SEED: &[u8] = b"fundraiser";
    pub const LEN: usize = core::mem::size_of::<Self>();

    #[inline(always)]
    pub fn key(&self) -> Pubkey {
        find_program_address(&[self.maker.as_ref(), Self::FUNDRAISER_SEED], &crate::ID).0
    }

    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    pub fn initialize(
        &mut self,
        Fundraiser {
            is_initialized,
            maker,
            mint_to_raise,
            target_amount,
            raised_amount,
            start_time,
            duration,
            bump,
        }: Fundraiser,
    ) {
        self.is_initialized = is_initialized;
        self.raised_amount = raised_amount;
        self.start_time = start_time;
        self.duration = duration;
        self.target_amount = target_amount;
        self.mint_to_raise = mint_to_raise;
        self.maker = maker;
        self.bump = bump;
    }
}
