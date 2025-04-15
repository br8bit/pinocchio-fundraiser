use pinocchio::{
    ProgramResult, account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};

use crate::instructions::{self, ProgramInstruction};

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match ProgramInstruction::unpack(discriminator)? {
        ProgramInstruction::Initialize => instructions::initialize::process(accounts, data),
        ProgramInstruction::Checker => instructions::checker::process(accounts, data),
        ProgramInstruction::Contribute => instructions::contribute::process(accounts, data),
        ProgramInstruction::Refund => instructions::refund::process(accounts, data),
    }
}
