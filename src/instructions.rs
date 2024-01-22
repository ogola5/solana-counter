use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use std::convert::TryInto;

// instructions.rs

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment(u32),
    Decrement(u32),
    Update(UpdateArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Increment(u32::from_le_bytes(rest.try_into().unwrap())),
            1 => Self::Decrement(u32::from_le_bytes(rest.try_into().unwrap())),
            2 => {
                let args = UpdateArgs::try_from_slice(rest)?;
                Self::Update(args)
            }
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
