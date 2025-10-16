use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::env;
use anyhow::Result;
use chrono::{Duration, Utc};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub user_type: String, // User type (customer, restaurant, delivery_rider)
    pub role: String, // Role field for compatibility  
    pub token_type: String, // Token type (access, refresh, reset)
    pub exp: i64,    // Expiration
    pub iat: i64,    // Issued at
}

// =======================
// Reset Token Claims
// =======================
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetClaims {
    pub sub: String, // user email
    pub exp: i64,
    pub otp: Option<String>,
}

const RESET_SECRET_KEY: &str = "RESET_SECRET"; // Can also use env var for production

// =======================
// JWT Functions
// =======================



// Generate access token
pub fn generate_access_token(user_id: String, user_type: String) -> Result<String> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let now = Utc::now();
    let expire = now + Duration::minutes(15);
    
    let claims = Claims {
        sub: user_id,
        user_type: user_type.clone(),
        role: user_type.clone(),
        token_type: "access".to_string(),
        exp: expire.timestamp(),
        iat: now.timestamp(),
    };              

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}

// Generate refresh token
pub fn generate_refresh_token(user_id: String, user_type: String) -> Result<String> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let now = Utc::now();
    let expire = now + Duration::days(7);
    
    let claims = Claims {
        sub: user_id,
        user_type: user_type.clone(),
        role: user_type.clone(),
        token_type: "refresh".to_string(),
        exp: expire.timestamp(),
        iat: now.timestamp(),
    };              

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok(token)
}

// Generate reset token
pub fn generate_reset_token(email: &str) -> Result<(String, String)> {
    use rand::{distributions::Uniform, Rng};

    let secret = env::var(RESET_SECRET_KEY).unwrap_or_else(|_| "super_secret_reset_key".to_string());
    let now = Utc::now();
    let expire = now + Duration::minutes(15);

    // Generate a 6-digit OTP
    let mut rng = rand::thread_rng();
    let range = Uniform::from(0..1_000_000);
    let otp_val = rng.sample(range);
    let otp = format!("{:06}", otp_val);

    let claims = ResetClaims {
        sub: email.to_string(),
        exp: expire.timestamp(),
        otp: Some(otp.clone()),
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))?;
    Ok((token, otp))
}

// Verify reset token
/// Verify reset token and optionally ensure provided OTP matches the token's OTP.
pub fn verify_reset_token(token: &str, otp: Option<&str>) -> Result<String> {
    let secret = env::var(RESET_SECRET_KEY).unwrap_or_else(|_| "super_secret_reset_key".to_string());

    let data = decode::<ResetClaims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    if let Some(expected_otp) = data.claims.otp {
        if let Some(provided) = otp {
            if provided != expected_otp {
                return Err(anyhow::anyhow!("Invalid OTP"));
            }
        } else {
            // No OTP provided but token contains one - require OTP for extra security
            return Err(anyhow::anyhow!("OTP required"));
        }
    }

    Ok(data.claims.sub)
}

// Legacy token generator
#[allow(dead_code)]
pub fn generate_token(user_id: String) -> Result<String> {
    generate_access_token(user_id, "customer".to_string())
}

// Generate access + refresh pair
pub fn generate_tokens(claims: &Claims) -> Result<(String, String)> {
    let access_token = generate_access_token(claims.sub.clone(), claims.user_type.clone())?;
    let refresh_token = generate_refresh_token(claims.sub.clone(), claims.user_type.clone())?;
    Ok((access_token, refresh_token))
}

// Agent tokens
pub fn generate_agent_tokens(user_id: String, user_type: String) -> Result<(String, String, i64)> {
    let access_token = generate_access_token(user_id.clone(), user_type.clone())?;
    let refresh_token = generate_refresh_token(user_id, user_type)?;
    let expires_in = 900;
    Ok((access_token, refresh_token, expires_in))
}

// Restaurant tokens
pub fn generate_restaurant_tokens(restaurant_id: i32) -> Result<(String, String, i64)> {
    let user_id = restaurant_id.to_string();
    let user_type = "restaurant".to_string();
    let access_token = generate_access_token(user_id.clone(), user_type.clone())?;
    let refresh_token = generate_refresh_token(user_id, user_type)?;
    let expires_in = 900;
    Ok((access_token, refresh_token, expires_in))
}

// Restaurant user tokens
pub fn generate_restaurant_user_tokens(restaurant_user_id: i32) -> Result<(String, String, i64)> {
    let user_id = restaurant_user_id.to_string();
    let user_type = "restaurant_user".to_string();
    let access_token = generate_access_token(user_id.clone(), user_type.clone())?;
    let refresh_token = generate_refresh_token(user_id, user_type)?;
    let expires_in = 900;
    Ok((access_token, refresh_token, expires_in))
}

// Verify token (general)
pub fn verify_token(token: &str) -> Result<Claims> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256))?;
    Ok(token_data.claims)
}

// Verify access token
pub fn verify_access_token(token: &str) -> Result<Claims> {
    let claims = verify_token(token)?;
    if claims.token_type != "access" {
        return Err(anyhow::anyhow!("Invalid token type"));
    }
    Ok(claims)
}

// Verify refresh token
pub fn verify_refresh_token(token: &str) -> Result<Claims> {
    let claims = verify_token(token)?;
    if claims.token_type != "refresh" {
        return Err(anyhow::anyhow!("Invalid token type"));
    }
    Ok(claims)
}
