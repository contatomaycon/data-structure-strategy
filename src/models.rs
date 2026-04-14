use serde::{Serialize, Deserialize};

pub type ProductId = usize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    #[serde(default)]
    pub brand: String,
    pub category: String,
    pub tags: Vec<String>,
}