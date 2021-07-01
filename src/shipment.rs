use std::fmt::{Debug, Display};

/// An object to represent the shipments
#[derive(Debug, Default)]
pub struct Shipment {
    date: String,
    pub size: String,
    pub service: String,
    pub ignored: bool,
    pub price: f64,
    pub discount: f64,
}

impl Shipment {
    /// Returns the YYYY-MM part of the shipment date
    pub fn get_year_month(&self) -> &str {
        let (left, _) = self.date.split_at(7);
        left
    }
}

impl From<String> for Shipment {
    fn from(line: String) -> Self {
        let mut shipment = Shipment::default();
        let line = line
            .split(" ")
            .map(|w| w.to_ascii_uppercase())
            .collect::<Vec<String>>();
        let get_column =
            |line: &Vec<String>, column: usize| line.get(column).unwrap_or(&"".to_string()).clone();
        shipment.date = get_column(&line, 0);
        shipment.size = get_column(&line, 1);
        shipment.service = get_column(&line, 2);
        // If the line does not have 3 columns we should ignore it
        if line.len() != 3 {
            shipment.ignored = true;
            return shipment;
        }
        match get_column(&line, 1).as_str() {
            "S" | "M" | "L" => (),
            _ => {
                shipment.ignored = true;
                return shipment;
            }
        }
        match get_column(&line, 2).as_str() {
            "MR" | "LP" => (),
            _ => {
                shipment.ignored = true;
                return shipment;
            }
        }
        shipment
    }
}

/// Formats the Shipment for printing
impl Display for Shipment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} ", self.date, self.size, self.service)?;
        if self.ignored {
            return write!(f, "Ignored");
        }
        write!(f, "{:.2} ", self.price)?;
        if self.discount == 0. {
            write!(f, "-")
        } else {
            write!(f, "{:.2}", self.discount)
        }
    }
}
