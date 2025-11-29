use ed25519_dalek::{Keypair, SecretKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationPolicy {
    pub rotation_interval_days: u32,
    pub max_key_age_days: u32,
}

#[derive(Debug, Clone)]
pub struct SecureKeyManager {
    active_keys: HashMap<Uuid, Keypair>,
    rotation_policy: KeyRotationPolicy,
}

impl SecureKeyManager {
    pub fn new(rotation_policy: KeyRotationPolicy) -> Self {
        Self {
            active_keys: HashMap::new(),
            rotation_policy,
        }
    }

    pub fn generate_user_keypair(&mut self, user_id: Uuid) -> Vec<u8> {
        let mut csprng = rand::rngs::OsRng {};
        let keypair = Keypair::generate(&mut csprng);
        let public_key = keypair.public.to_bytes().to_vec();
        self.active_keys.insert(user_id, keypair);
        public_key
    }

    pub fn get_keypair(&self, user_id: &Uuid) -> Option<&Keypair> {
        self.active_keys.get(user_id)
    }

    pub fn rotate_key(&mut self, user_id: Uuid) -> Vec<u8> {
        self.generate_user_keypair(user_id)
    }
}