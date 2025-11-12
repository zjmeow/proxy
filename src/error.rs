use pinocchio::program_error::ProgramError;
#[derive(Debug)]
pub enum ArbError {
    NoProfit,
    NoPool,
    SwapZero,
    ProfitNoEnough,
}
impl From<ArbError> for ProgramError {
    fn from(e: ArbError) -> Self {
        ProgramError::Custom(e as u32)
    }
}