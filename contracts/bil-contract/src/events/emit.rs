use soroban_sdk::{ Env, Symbol, Address };
use super::types::{ InteractionAdded, ReviewDisputed, AdminTransferred };

pub fn emit_interaction_added(env: &Env, vendor: Symbol, score: u32, user_id: Symbol) {
    (InteractionAdded { vendor, score, user_id }).publish(env);
}

pub fn emit_review_disputed(env: &Env, vendor: Symbol) {
    (ReviewDisputed { vendor }).publish(env);
}

pub fn emit_admin_transferred(env: &Env, old_admin: Address, new_admin: Address) {
    (AdminTransferred { old_admin, new_admin }).publish(env);
}
