use pinocchio::program_error::ProgramError;
#[derive(Clone, PartialEq)]
pub enum MyProgramError {
    NoProfit,
    InvalidInstructionData,
    PdaMismatch,
    InvalidOwner,
}

impl From<MyProgramError> for ProgramError {
    fn from(e: MyProgramError) -> Self {
        Self::Custom(e as u32)
    }
}
