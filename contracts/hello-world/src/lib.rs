#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Bytes, Symbol, symbol_short};

// Key to store message against sender
#[contracttype]
pub enum MessageKey {
    EncryptedMessage(Address),
}

#[contract]
pub struct MessageEncryptor;

#[contractimpl]
impl MessageEncryptor {
    // Save encrypted message
    pub fn store_message(env: Env, user: Address, message: Bytes) {
        env.storage().instance().set(&MessageKey::EncryptedMessage(user), &message);
    }

    // Retrieve stored encrypted message
    pub fn get_message(env: Env, user: Address) -> Bytes {
        env.storage().instance().get(&MessageKey::EncryptedMessage(user)).unwrap_or(Bytes::from_array(&env, &[0u8; 0]))
    }
}
