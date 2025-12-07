use solana_pubkey::Pubkey;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ModelId(pub u64);

#[derive(Clone, Debug)]
pub struct ModelMetadata {
    pub id: ModelId,
    pub owner: Pubkey,          // validator / provider
    pub name: String,
    pub description: String,
    pub version: String,

    // Optional: dacă vrei să păstrezi și hash:
    // pub hash: [u8; 32],

    // Proof-of-Models:
    pub stake_lamports: u64,    // cât stake susține modelul
    pub quality_score: u32,     // scor agregat PoM
    pub last_epoch_updated: u64,
}






