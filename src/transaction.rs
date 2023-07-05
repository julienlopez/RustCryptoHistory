#[derive(Debug, PartialEq)]
pub enum Type {
    Bought,
    Sold,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    pub currency : String,
    pub r#type: Type,
    pub amount: f64,
    pub total_value: f64,
    pub price: f64,
    pub timestamp : String,
}
