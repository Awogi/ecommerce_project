//! Shared types for the School Uniform Store
use serde::{Deserialize, Serialize};

/// Minimal product/uniform type shared between backend and frontend
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub grade: String,
    pub size: String,
    pub price_cents: u64,
}

impl Product {
    pub fn new(id: u64, name: impl Into<String>, grade: impl Into<String>, size: impl Into<String>, price_cents: u64) -> Self {
        Self { id, name: name.into(), grade: grade.into(), size: size.into(), price_cents }
    }
}

pub type ProductList = Vec<Product>;
