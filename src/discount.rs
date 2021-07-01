use crate::{repository::Repository, shipment::Shipment};

/// Interface for discount implementations
pub trait Discount {
    /// Applies a discount strategy to the shipment and
    /// substracts the discount from the available discounts
    fn apply(
        &mut self,
        shipment: &mut Shipment,
        available_discount: &mut f64,
        price_repo: &dyn Repository,
    );
}

/// Discount strategy for Small shipments
pub struct SDiscount;

impl Discount for SDiscount {
    fn apply(
        &mut self,
        shipment: &mut Shipment,
        available_discount: &mut f64,
        price_repo: &dyn Repository,
    ) {
        match shipment.size.as_str() {
            "S" => {
                let min_price = price_repo.get_min_price_by_size("S");
                let max_price = price_repo.get_price("S", &shipment.service);
                let discount = max_price - min_price;
                let discount = if *available_discount >= discount {
                    discount
                } else {
                    *available_discount
                };
                *available_discount -= discount;
                shipment.discount = discount;
                shipment.price = max_price - discount;
            }
            _ => (),
        }
    }
}

/// Discount strategy for Large shipments
#[derive(Default)]
pub struct LDiscount {
    purchases: u64,
}

impl LDiscount {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Discount for LDiscount {
    fn apply(
        &mut self,
        shipment: &mut Shipment,
        available_discount: &mut f64,
        price_repo: &dyn Repository,
    ) {
        match shipment.size.as_str() {
            "L" => {
                let price = price_repo.get_price("L", &shipment.service);
                let mut discount = 0.;
                if shipment.service == "LP" {
                    self.purchases += 1;
                    if self.purchases % 3 == 0 {
                        discount = if *available_discount >= price {
                            price
                        } else {
                            0.
                        };
                        *available_discount -= discount;
                    }
                }
                shipment.discount = discount;
                shipment.price = price - discount;
            }
            _ => (),
        }
    }
}
