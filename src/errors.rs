use pinocchio::program_error::ProgramError;

pub enum FundraiserError {
    //#[msg("The amount to raise has not been met")]
    TargetNotMet,
    //#[msg("The amount to raise has been achieved")]
    TargetMet,
    //#[msg("The contribution is too big")]
    ContributionTooBig,
    //#[msg("The contribution is too small")]
    ContributionTooSmall,
    //#[msg("The maximum amount to contribute has been reached")]
    MaximumContributionsReached,
    //#[msg("The fundraiser has not ended yet")]
    FundraiserNotEnded,
    //#[msg("The fundraiser has ended")]
    FundraiserEnded,
    //#[msg("Invalid total amount. i should be bigger than 3")]
    InvalidAmount,
}

impl From<FundraiserError> for ProgramError {
    fn from(error: FundraiserError) -> Self {
        Self::Custom(error as u32)
    }
}
