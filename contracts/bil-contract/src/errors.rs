use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    Uninitialized = 2,
    Unauthorized = 3,
    InvalidScore = 4,
    InvalidSignature = 5,
    DuplicateReview = 6,
    ReviewAlreadyExists = 7,
    ReviewNotFound = 8,
    ReviewAlreadyDisputed = 9,
    VendorNotFound = 10,
    NonceUsed = 11,
}
