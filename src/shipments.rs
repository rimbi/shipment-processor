use std::fmt::Display;

use crate::{discount::Discount, repository::Repository, shipment::Shipment};

/// A monthly window of shipments
pub struct MonthWindow {
    available_discount: f64,
    pub shipments: Vec<Shipment>,
    price_repo: Box<dyn Repository>,
}

impl MonthWindow {
    pub fn new(available_discount: f64, price_repo: Box<dyn Repository>) -> Self {
        Self {
            available_discount,
            shipments: vec![],
            price_repo,
        }
    }

    /// Applies discounts to the shipments of a month
    pub fn apply_discounts(&mut self, discounts: &mut Vec<Box<dyn Discount>>) {
        for shipment in &mut self.shipments.iter_mut().filter(|s| !s.ignored) {
            shipment.price = self.price_repo.get_price(&shipment.size, &shipment.service);
            for discount in &mut *discounts {
                discount.apply(shipment, &mut self.available_discount, &*self.price_repo);
            }
        }
    }
}

/// Formats the MonthWindow for printing
impl Display for MonthWindow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self
            .shipments
            .iter()
            .map(|s| format!("{}", s))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", out)
    }
}

pub struct Shipments {
    pub month_windows: Vec<MonthWindow>,
}

/// Formats the Shipments for printing
impl Display for Shipments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self
            .month_windows
            .iter()
            .map(|mw| format!("{}", mw))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", x)
    }
}
