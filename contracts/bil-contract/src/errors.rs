use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    Unauthorized = 2,
    InvalidScore = 3,
    DuplicateReview = 4,
    ReviewAlreadyExists = 5,
    InvalidSignature = 6,
    NonceUsed = 7,
    VendorNotFound = 8,
    ReviewNotFound = 9,
    ReviewAlreadyDisputed = 10,
}
