use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum AiExecutorInstruction {
    /// Clientul cere un job AI.
    /// - model_id: identificatorul modelului din ai_models.rs
    /// - payload: inputul serializat (prompt, features, etc.)
    SubmitJob {
        model_id: u64,
        payload: Vec<u8>,
    },
}
