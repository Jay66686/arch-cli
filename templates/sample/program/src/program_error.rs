use thiserror::Error;

/// Reasons the program may fail
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ProgramError {
    /// Allows on-chain programs to implement program-specific error types and see them returned
    /// by the Solana runtime. A program-specific error may be any type that is represented as
    /// or serialized to a u32 integer.
    #[error("Custom program error: {0:#x}")]
    Custom(u32),
    #[error("The arguments provided to a program instruction were invalid")]
    InvalidArgument,
    #[error("An instruction's data contents was invalid")]
    InvalidInstructionData,
    #[error("An account's data contents was invalid")]
    InvalidAccountData,
    #[error("An account's data was too small")]
    AccountDataTooSmall,
    #[error("An account's balance was too small to complete the instruction")]
    InsufficientFunds,
    #[error("The account did not have the expected program id")]
    IncorrectProgramId,
    #[error("A signature was required but not found")]
    MissingRequiredSignature,
    #[error("An initialize instruction was sent to an account that has already been initialized")]
    AccountAlreadyInitialized,
    #[error("An attempt to operate on an account that hasn't been initialized")]
    UninitializedAccount,
    #[error("The instruction expected additional account keys")]
    NotEnoughAccountKeys,
    #[error("Failed to borrow a reference to account data, already borrowed")]
    AccountBorrowFailed,
    #[error("Length of the seed is too long for address generation")]
    MaxSeedLengthExceeded,
    #[error("Provided seeds do not result in a valid address")]
    InvalidSeeds,
    #[error("IO Error: {0}")]
    BorshIoError(String),
    #[error("Unsupported sysvar")]
    IllegalOwner,
    #[error("Accounts data allocations exceeded the maximum allowed per transaction")]
    MaxAccountsDataAllocationsExceeded,
    #[error("Account data reallocation was invalid")]
    InvalidRealloc,
    #[error("Instruction trace length exceeded the maximum allowed per transaction")]
    MaxInstructionTraceLengthExceeded,
    #[error("Builtin programs must consume compute units")]
    BuiltinProgramsMustConsumeComputeUnits,
    #[error("Invalid account owner")]
    InvalidAccountOwner,
    #[error("Program arithmetic overflowed")]
    ArithmeticOverflow,
    #[error("Account is immutable")]
    Immutable,
    #[error("Incorrect authority provided")]
    IncorrectAuthority,
}

/// Builtin return values occupy the upper 32 bits
pub const BUILTIN_BIT_SHIFT: usize = 32;
macro_rules! to_builtin {
    ($error:expr) => {
        ($error as u64) << BUILTIN_BIT_SHIFT
    };
}

pub const CUSTOM_ZERO: u64 = to_builtin!(1);
pub const INVALID_ARGUMENT: u64 = to_builtin!(2);
pub const INVALID_INSTRUCTION_DATA: u64 = to_builtin!(3);
pub const INVALID_ACCOUNT_DATA: u64 = to_builtin!(4);
pub const ACCOUNT_DATA_TOO_SMALL: u64 = to_builtin!(5);
pub const INSUFFICIENT_FUNDS: u64 = to_builtin!(6);
pub const INCORRECT_PROGRAM_ID: u64 = to_builtin!(7);
pub const MISSING_REQUIRED_SIGNATURES: u64 = to_builtin!(8);
pub const ACCOUNT_ALREADY_INITIALIZED: u64 = to_builtin!(9);
pub const UNINITIALIZED_ACCOUNT: u64 = to_builtin!(10);
pub const NOT_ENOUGH_ACCOUNT_KEYS: u64 = to_builtin!(11);
pub const ACCOUNT_BORROW_FAILED: u64 = to_builtin!(12);
pub const MAX_SEED_LENGTH_EXCEEDED: u64 = to_builtin!(13);
pub const INVALID_SEEDS: u64 = to_builtin!(14);
pub const BORSH_IO_ERROR: u64 = to_builtin!(15);
pub const ACCOUNT_NOT_RENT_EXEMPT: u64 = to_builtin!(16);
pub const UNSUPPORTED_SYSVAR: u64 = to_builtin!(17);
pub const ILLEGAL_OWNER: u64 = to_builtin!(18);
pub const MAX_ACCOUNTS_DATA_ALLOCATIONS_EXCEEDED: u64 = to_builtin!(19);
pub const INVALID_ACCOUNT_DATA_REALLOC: u64 = to_builtin!(20);
pub const MAX_INSTRUCTION_TRACE_LENGTH_EXCEEDED: u64 = to_builtin!(21);
pub const BUILTIN_PROGRAMS_MUST_CONSUME_COMPUTE_UNITS: u64 = to_builtin!(22);
pub const INVALID_ACCOUNT_OWNER: u64 = to_builtin!(23);
pub const ARITHMETIC_OVERFLOW: u64 = to_builtin!(24);
pub const IMMUTABLE: u64 = to_builtin!(25);
pub const INCORRECT_AUTHORITY: u64 = to_builtin!(26);
// Warning: Any new program errors added here must also be:
// - Added to the below conversions
// - Added as an equivalent to InstructionError
// - Be featureized in the BPF loader to return `InstructionError::InvalidError`
//   until the feature is activated

impl From<ProgramError> for u64 {
    fn from(error: ProgramError) -> Self {
        match error {
            ProgramError::InvalidArgument => INVALID_ARGUMENT,
            ProgramError::InvalidInstructionData => INVALID_INSTRUCTION_DATA,
            ProgramError::InvalidAccountData => INVALID_ACCOUNT_DATA,
            ProgramError::AccountDataTooSmall => ACCOUNT_DATA_TOO_SMALL,
            ProgramError::InsufficientFunds => INSUFFICIENT_FUNDS,
            ProgramError::IncorrectProgramId => INCORRECT_PROGRAM_ID,
            ProgramError::MissingRequiredSignature => MISSING_REQUIRED_SIGNATURES,
            ProgramError::AccountAlreadyInitialized => ACCOUNT_ALREADY_INITIALIZED,
            ProgramError::UninitializedAccount => UNINITIALIZED_ACCOUNT,
            ProgramError::NotEnoughAccountKeys => NOT_ENOUGH_ACCOUNT_KEYS,
            ProgramError::AccountBorrowFailed => ACCOUNT_BORROW_FAILED,
            ProgramError::MaxSeedLengthExceeded => MAX_SEED_LENGTH_EXCEEDED,
            ProgramError::InvalidSeeds => INVALID_SEEDS,
            ProgramError::BorshIoError(_) => BORSH_IO_ERROR,
            ProgramError::IllegalOwner => ILLEGAL_OWNER,
            ProgramError::MaxAccountsDataAllocationsExceeded => {
                MAX_ACCOUNTS_DATA_ALLOCATIONS_EXCEEDED
            }
            ProgramError::InvalidRealloc => INVALID_ACCOUNT_DATA_REALLOC,
            ProgramError::MaxInstructionTraceLengthExceeded => {
                MAX_INSTRUCTION_TRACE_LENGTH_EXCEEDED
            }
            ProgramError::BuiltinProgramsMustConsumeComputeUnits => {
                BUILTIN_PROGRAMS_MUST_CONSUME_COMPUTE_UNITS
            }
            ProgramError::InvalidAccountOwner => INVALID_ACCOUNT_OWNER,
            ProgramError::ArithmeticOverflow => ARITHMETIC_OVERFLOW,
            ProgramError::Immutable => IMMUTABLE,
            ProgramError::IncorrectAuthority => INCORRECT_AUTHORITY,
            ProgramError::Custom(error) => {
                if error == 0 {
                    CUSTOM_ZERO
                } else {
                    error as u64
                }
            }
        }
    }
}

impl From<u64> for ProgramError {
    fn from(error: u64) -> Self {
        match error {
            CUSTOM_ZERO => Self::Custom(0),
            INVALID_ARGUMENT => Self::InvalidArgument,
            INVALID_INSTRUCTION_DATA => Self::InvalidInstructionData,
            INVALID_ACCOUNT_DATA => Self::InvalidAccountData,
            ACCOUNT_DATA_TOO_SMALL => Self::AccountDataTooSmall,
            INSUFFICIENT_FUNDS => Self::InsufficientFunds,
            INCORRECT_PROGRAM_ID => Self::IncorrectProgramId,
            MISSING_REQUIRED_SIGNATURES => Self::MissingRequiredSignature,
            ACCOUNT_ALREADY_INITIALIZED => Self::AccountAlreadyInitialized,
            UNINITIALIZED_ACCOUNT => Self::UninitializedAccount,
            NOT_ENOUGH_ACCOUNT_KEYS => Self::NotEnoughAccountKeys,
            ACCOUNT_BORROW_FAILED => Self::AccountBorrowFailed,
            MAX_SEED_LENGTH_EXCEEDED => Self::MaxSeedLengthExceeded,
            INVALID_SEEDS => Self::InvalidSeeds,
            BORSH_IO_ERROR => Self::BorshIoError("Unknown".to_string()),
            ILLEGAL_OWNER => Self::IllegalOwner,
            MAX_ACCOUNTS_DATA_ALLOCATIONS_EXCEEDED => Self::MaxAccountsDataAllocationsExceeded,
            INVALID_ACCOUNT_DATA_REALLOC => Self::InvalidRealloc,
            MAX_INSTRUCTION_TRACE_LENGTH_EXCEEDED => Self::MaxInstructionTraceLengthExceeded,
            BUILTIN_PROGRAMS_MUST_CONSUME_COMPUTE_UNITS => {
                Self::BuiltinProgramsMustConsumeComputeUnits
            }
            INVALID_ACCOUNT_OWNER => Self::InvalidAccountOwner,
            ARITHMETIC_OVERFLOW => Self::ArithmeticOverflow,
            IMMUTABLE => Self::Immutable,
            INCORRECT_AUTHORITY => Self::IncorrectAuthority,
            _ => Self::Custom(error as u32),
        }
    }
}
