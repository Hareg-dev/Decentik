use dtt_core::{ports::AuthService, domain::User};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct JwtAuthService {
    secret: String,
}

impl JwtAuthService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl AuthService<String> for JwtAuthService {
    async fn authenticate(&self, token: &str) -> Result<User, String> {
        let validation = Validation::new(Algorithm::HS256);
        let key = DecodingKey::from_secret(self.secret.as_ref());
        
        let token_data = decode::<Claims>(token, &key, &validation)
            .map_err(|e| e.to_string())?;
        
        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|e| e.to_string())?;
        
        Ok(User {
            id: user_id,
            handle: "authenticated_user".to_string(),
        })
    }

    async fn generate_token(&self, user_id: Uuid) -> Result<String, String> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
        };

        let key = EncodingKey::from_secret(self.secret.as_ref());
        encode(&Header::default(), &claims, &key)
            .map_err(|e| e.to_string())
    }

    async fn hash_password(&self, password: &str) -> Result<String, String> {
        hash(password, DEFAULT_COST)
            .map_err(|e| e.to_string())
    }

    async fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String> {
        verify(password, hash)
            .map_err(|e| e.to_string())
    }
}