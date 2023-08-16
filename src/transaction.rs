#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Bought,
    Sold,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {
    pub currency: String,
    pub r#type: Type,
    pub amount: f64,
    pub total_value: f64,
    pub price: f64,
    pub timestamp: String,
}
