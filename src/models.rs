#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub price: f64,
}

impl Product {
    pub fn new(id: u32, name: &str, brand: &str, category: &str, price: f64) -> Self {
        Product {
            id,
            name: name.to_string(),
            brand: brand.to_string(),
            category: category.to_string(),
            price,
        }
    }
}