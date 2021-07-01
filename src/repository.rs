use std::{collections::HashMap, fmt::Debug};

/// Repository interface for shipment prices
pub trait Repository: Debug {
    /// Returns the minimum price for a size from all service providers
    fn get_min_price_by_size(&self, size: &str) -> f64;
    /// Given size and service returns the shipment price
    fn get_price(&self, size: &str, service: &str) -> f64;
}

/// A static Price Repository implementation
#[derive(Debug)]
pub struct PriceRepository {
    price_map: HashMap<&'static str, HashMap<&'static str, f64>>,
}

impl PriceRepository {
    pub fn new() -> Self {
        let a = [("S", "1.5"), ("M", "4.9"), ("L", "6.9")]
            .iter()
            .cloned()
            .map(|(k, v)| (k, v.parse::<f64>().unwrap()))
            .collect();
        let b = [("S", "2"), ("M", "3"), ("L", "4")]
            .iter()
            .cloned()
            .map(|(k, v)| (k, v.parse::<f64>().unwrap()))
            .collect();
        let mut price_map = HashMap::new();
        price_map.insert("LP", a);
        price_map.insert("MR", b);
        Self { price_map }
    }
}

impl Repository for PriceRepository {
    fn get_min_price_by_size(&self, size: &str) -> f64 {
        let p1 = self.price_map["MR"][size];
        let p2 = self.price_map["LP"][size];
        if p1 < p2 {
            p1
        } else {
            p2
        }
    }

    fn get_price(&self, size: &str, service: &str) -> f64 {
        self.price_map[service][size]
    }
}
