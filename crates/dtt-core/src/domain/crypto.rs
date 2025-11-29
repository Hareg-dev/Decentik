use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoSignature {
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct KeyPair {
    keypair: Keypair,
}

impl KeyPair {
    pub fn generate() -> Self {
        let mut csprng = rand::rngs::OsRng {};
        Self {
            keypair: Keypair::generate(&mut csprng),
        }
    }

    pub fn sign(&self, data: &[u8]) -> CryptoSignature {
        let hash = Sha256::digest(data);
        let signature = self.keypair.sign(&hash);
        
        CryptoSignature {
            signature: signature.to_bytes().to_vec(),
            public_key: self.keypair.public.to_bytes().to_vec(),
        }
    }

    pub fn public_key(&self) -> Vec<u8> {
        self.keypair.public.to_bytes().to_vec()
    }
}

pub fn verify_signature(data: &[u8], sig: &CryptoSignature) -> bool {
    let hash = Sha256::digest(data);
    
    if let (Ok(public_key), Ok(signature)) = (
        PublicKey::from_bytes(&sig.public_key),
        Signature::from_bytes(&sig.signature.try_into().unwrap_or([0; 64]))
    ) {
        public_key.verify(&hash, &signature).is_ok()
    } else {
        false
    }
}