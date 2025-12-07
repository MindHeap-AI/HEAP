#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AiJobId(pub u64);

use crate::ai_executor::AiExecutor;
use crate::ai_models::ModelId;

/// Reprezintă un job AI ce trebuie executat.
pub struct AiJob {
    pub model_id: ModelId,
    pub payload: Vec<u8>,
}

/// Scheduler simplu care primește joburi și le trimite la executor.
pub struct AiScheduler {
    pub executor: AiExecutor,
}

impl AiScheduler {
    pub fn new(executor: AiExecutor) -> Self {
        Self { executor }
    }

    pub fn process_job(&self, job: AiJob) {
        self.executor.submit_job(crate::ai_executor::AiJob {
            job_id: AiJobId(0),
            slot: 0,
            tx_signature: solana_signature::Signature::default(),
            model_id: job.model_id,
            payload: job.payload,
        });
    }
}