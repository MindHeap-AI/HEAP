use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

mod instruction;
use instruction::AiExecutorInstruction;

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let _accounts_iter = &mut accounts.iter();

    let ix = AiExecutorInstruction::try_from_slice(input)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match ix {
        AiExecutorInstruction::SubmitJob { model_id, payload } => {
            msg!(
                "AI job submitted: model_id={}, payload_len={}",
                model_id,
                payload.len()
            );
            // Nu facem aici AI; doar log / eventual scriem într-un cont.
            // Runtime-ul va scana log-urile și va construi AiJob din ele.
        }
    }

    Ok(())
}
