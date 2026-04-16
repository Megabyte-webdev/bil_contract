use soroban_sdk::{ contracttype, BytesN, Symbol };

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Operator,
    Vendor(Symbol),
    Review(BytesN<32>),
    UserReview(Symbol, Symbol),
    UsedNonce(BytesN<32>),
}
