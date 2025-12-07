use borsh::BorshDeserialize;
use crate::ai_models::ModelId;
use crate::ai_scheduler::AiJobId;
use solana_signature::Signature;
use tracing::info;

#[derive(Clone, Debug)]
pub struct AiExecutionRequestId(pub u64);

#[derive(Clone, Debug)]
pub struct AiJob {
    pub job_id: AiJobId,
    pub slot: u64,
    pub tx_signature: Signature,
    pub model_id: ModelId,
    pub payload: Vec<u8>,
}

pub struct AiExecutor;

impl AiExecutor {
    pub fn submit_job(&self, job: AiJob) {
        // MVP: doar logăm
        info!("Submitting AI job {:?}", job.job_id);
    }

    pub fn process_completed_jobs(&self) {
        // MVP: gol, doar schelet
    }
}

// enum folosit DOAR în runtime, pentru decodare
#[derive(BorshDeserialize, Debug, Clone)]
pub enum AiExecutorInstruction {
    SubmitJob { model_id: u64, payload: Vec<u8> },
}

pub fn decode_ai_instruction(data: &[u8]) -> Result<(u64, Vec<u8>), ()> {
    let ix = AiExecutorInstruction::try_from_slice(data).map_err(|_| ())?;
    match ix {
        AiExecutorInstruction::SubmitJob { model_id, payload } => Ok((model_id, payload)),
    }
}
