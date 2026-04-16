use soroban_sdk::{ contractevent, Address, Symbol };

#[contractevent]
pub struct InteractionAdded {
    pub vendor: Symbol,
    pub score: u32,
    pub user_id: Symbol,
}

#[contractevent]
pub struct ReviewDisputed {
    pub vendor: Symbol,
}

#[contractevent]
pub struct AdminTransferred {
    pub old_admin: Address,
    pub new_admin: Address,
}
