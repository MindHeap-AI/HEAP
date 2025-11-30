use solana_pubkey::Pubkey;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ModelId(pub u64);

#[derive(Clone, Debug)]
pub struct ModelMetadata {
    pub id: ModelId,
    pub owner: Pubkey,
    pub name: String,
    pub hash: [u8; 32],
}





