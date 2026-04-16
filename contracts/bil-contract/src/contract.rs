#![no_std]

use soroban_sdk::{ contract, contractimpl, panic_with_error, Address, BytesN, Env, Symbol };

use crate::{
    errors::ContractError,
    storage::DataKey,
    types::{ InteractionProof, VendorProfile },
    events::emit::{ emit_admin_transferred, emit_interaction_added, emit_review_disputed },
};

#[contract]
pub struct BouwnceReputationContract;

#[contractimpl]
impl BouwnceReputationContract {
    pub fn initialize(env: Env, admin: Address, operator: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic_with_error!(&env, ContractError::AlreadyInitialized);
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Operator, &operator);
    }

    pub fn add_interaction(
        env: Env,
        operator: Address,
        user_id: Symbol,
        vendor_id: Symbol,
        score: u32,
        review_hash: BytesN<32>,
        nonce: BytesN<32>
    ) {
        Self::require_operator(&env, &operator);

        if score == 0 || score > 5 {
            panic_with_error!(&env, ContractError::InvalidScore);
        }

        if env.storage().persistent().has(&DataKey::UsedNonce(nonce.clone())) {
            panic_with_error!(&env, ContractError::NonceUsed);
        }

        env.storage().persistent().set(&DataKey::UsedNonce(nonce.clone()), &true);

        let user_vendor_key = DataKey::UserReview(user_id.clone(), vendor_id.clone());

        if env.storage().persistent().has(&user_vendor_key) {
            panic_with_error!(&env, ContractError::DuplicateReview);
        }

        env.storage().persistent().set(&user_vendor_key, &true);

        if env.storage().persistent().has(&DataKey::Review(review_hash.clone())) {
            panic_with_error!(&env, ContractError::ReviewAlreadyExists);
        }

        let review = InteractionProof {
            user: user_id.clone(),
            vendor: vendor_id.clone(),
            score,
            review_hash: review_hash.clone(),
            timestamp: env.ledger().timestamp(),
            disputed: false,
        };

        env.storage().persistent().set(&DataKey::Review(review_hash.clone()), &review);

        let mut vendor: VendorProfile = env
            .storage()
            .persistent()
            .get(&DataKey::Vendor(vendor_id.clone()))
            .unwrap_or(VendorProfile {
                total_interactions: 0,
                reputation_score: 0,
                verified_reviews: 0,
                disputed_reviews: 0,
                badge_level: Symbol::new(&env, "NEW"),
            });

        vendor.total_interactions += 1;
        vendor.reputation_score += score;
        vendor.verified_reviews += 1;
        vendor.badge_level = Self::calculate_badge(&env, vendor.total_interactions);

        env.storage().persistent().set(&DataKey::Vendor(vendor_id.clone()), &vendor);

        emit_interaction_added(&env, vendor_id, score, user_id);
    }

    pub fn get_vendor(env: Env, vendor_id: Symbol) -> VendorProfile {
        env.storage()
            .persistent()
            .get(&DataKey::Vendor(vendor_id))
            .unwrap_or_else(|| panic_with_error!(&env, ContractError::VendorNotFound))
    }

    pub fn get_review(env: Env, review_hash: BytesN<32>) -> InteractionProof {
        env.storage()
            .persistent()
            .get(&DataKey::Review(review_hash))
            .unwrap_or_else(|| panic_with_error!(&env, ContractError::ReviewNotFound))
    }

    pub fn dispute_review(env: Env, admin: Address, review_hash: BytesN<32>) {
        Self::require_admin(&env, &admin);

        let mut review: InteractionProof = env
            .storage()
            .persistent()
            .get(&DataKey::Review(review_hash.clone()))
            .unwrap_or_else(|| panic_with_error!(&env, ContractError::ReviewNotFound));

        if review.disputed {
            panic_with_error!(&env, ContractError::ReviewAlreadyDisputed);
        }

        review.disputed = true;

        env.storage().persistent().set(&DataKey::Review(review_hash.clone()), &review);

        emit_review_disputed(&env, review.vendor);
    }

    pub fn transfer_admin(env: Env, admin: Address, new_admin: Address) {
        Self::require_admin(&env, &admin);

        new_admin.require_auth();

        let old_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();

        env.storage().instance().set(&DataKey::Admin, &new_admin);

        emit_admin_transferred(&env, old_admin, new_admin);
    }

    pub fn update_operator(env: Env, admin: Address, new_operator: Address) {
        Self::require_admin(&env, &admin);

        new_operator.require_auth();

        env.storage().instance().set(&DataKey::Operator, &new_operator);
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    pub fn get_operator(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Operator).unwrap()
    }

    fn require_admin(env: &Env, caller: &Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();

        if admin != *caller {
            panic_with_error!(env, ContractError::Unauthorized);
        }

        caller.require_auth();
    }

    fn require_operator(env: &Env, caller: &Address) {
        let operator: Address = env.storage().instance().get(&DataKey::Operator).unwrap();

        if operator != *caller {
            panic_with_error!(env, ContractError::Unauthorized);
        }

        caller.require_auth();
    }

    fn calculate_badge(env: &Env, count: u32) -> Symbol {
        if count >= 200 {
            Symbol::new(env, "ELITE")
        } else if count >= 50 {
            Symbol::new(env, "PRO")
        } else if count >= 10 {
            Symbol::new(env, "TRUSTED")
        } else {
            Symbol::new(env, "NEW")
        }
    }
}
