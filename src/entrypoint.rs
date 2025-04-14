use pinocchio::{
    ProgramResult, account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};

use crate::instructions::{self, ProgramInstruction};

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, rest) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match ProgramInstruction::unpack(discriminator)? {
        ProgramInstruction::Initialize => instructions::initialize::process(accounts, rest),
        ProgramInstruction::Checker => instructions::checker::process(accounts, rest),
        ProgramInstruction::Contribute => instructions::contribute::process(accounts, rest),
        ProgramInstruction::Refund => instructions::refund::process(accounts, rest),
    }
}
