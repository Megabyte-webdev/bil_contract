use soroban_sdk::{ contracttype, BytesN, Symbol };

#[derive(Clone)]
#[contracttype]
pub struct InteractionProof {
    pub user: Symbol,
    pub vendor: Symbol,
    pub score: u32,
    pub review_hash: BytesN<32>,
    pub timestamp: u64,
    pub disputed: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct VendorProfile {
    pub total_interactions: u32,
    pub reputation_score: u32,
    pub verified_reviews: u32,
    pub disputed_reviews: u32,
    pub badge_level: Symbol,
}
