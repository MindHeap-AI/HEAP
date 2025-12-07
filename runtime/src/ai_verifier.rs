use crate::ai_models::ModelId;

pub type ModelQualityScore = u32;

pub struct VerificationResult {
    pub model_id: ModelId,
    pub score_delta: ModelQualityScore,
}

pub struct AiVerifier;

impl AiVerifier {
    pub fn verify_job_output(
        _job_id: u64,
        _expected_behavior: &[u8],
        _model_output: &[u8],
    ) -> VerificationResult {
        // MVP: scor fix 1, doar să curgă pipeline-ul
        VerificationResult {
            model_id: ModelId(0),
            score_delta: 1,
        }
    }
}
