use pinocchio::program_error::ProgramError;

#[repr(u8)]
pub enum ProgramInstruction {
    Initialize,
    Checker,
    Contribute,
    Refund,
}

impl ProgramInstruction {
    pub fn unpack(input: &u8) -> Result<Self, ProgramError> {
        Ok(match *input {
            0 => Self::Initialize,
            1 => Self::Contribute,
            2 => Self::Refund,
            3 => Self::Checker,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
