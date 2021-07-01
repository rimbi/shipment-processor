mod discount;
mod repository;
mod shipment;
mod shipments;

use discount::{Discount, LDiscount, SDiscount};
use repository::PriceRepository;
use shipment::Shipment;
use shipments::{MonthWindow, Shipments};

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const MAX_DISCOUNT: f64 = 10.;

fn process_shipments(data: Box<dyn std::io::Read>) -> Shipments {
    let reader = BufReader::new(data);
    // maps input data to a list of MonthWindow objects each of which holds
    // the shipment data in that month.
    let mut month_windows = reader
        .lines()
        .flat_map(|line| line.map(|line| line.trim().to_string()))
        .filter(|line| !line.is_empty())
        .map(Shipment::from)
        .fold(vec![], |mut months: Vec<MonthWindow>, shipment| {
            if months.last().is_none() {
                months.push(MonthWindow::new(
                    MAX_DISCOUNT,
                    Box::new(PriceRepository::new()),
                ))
            } else if shipment.get_year_month()
                != months
                    .last()
                    .unwrap()
                    .shipments
                    .last()
                    .unwrap()
                    .get_year_month()
            {
                months.push(MonthWindow::new(
                    MAX_DISCOUNT,
                    Box::new(PriceRepository::new()),
                ));
            }
            months.last_mut().unwrap().shipments.push(shipment);
            months
        });

    // Apply the discounts to MonthWindows and print them
    let mut discounts: Vec<Box<dyn Discount>> =
        vec![Box::new(SDiscount {}), Box::new(LDiscount::new())];
    month_windows.iter_mut().for_each(|month_window| {
        month_window.apply_discounts(&mut discounts);
    });
    Shipments { month_windows }
}
// Assumptions:
// - Dates always exist in the input file and they are are in the YYYY-MM-DD format
// - Records in the input file are sorted with regards to date

fn main() {
    let input_file = std::env::args().nth(1).unwrap_or("input.txt".to_string());
    let input_file =
        File::open(&input_file).expect(&format!("Error: Can't open the input file {}", input_file));

    let shipments = process_shipments(Box::new(input_file));
    println!("{}", shipments);
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn trim_data(data: &str) -> String {
        data.lines()
            .filter_map(|line| {
                let line = line.trim();
                if line == "" {
                    return None;
                } else {
                    Some(line)
                }
            })
            .collect::<Vec<&str>>()
            .join("\n")
    }

    #[test]
    fn small_shipment_should_work() {
        let input = r#"
        2015-02-01 S MR
        "#;
        let shipments = process_shipments(Box::new(input.as_bytes()));
        let expected = r#"
        2015-02-01 S MR 1.50 0.50
        "#;
        assert_eq!(trim_data(expected), format!("{}", shipments));
    }

    #[test]
    fn small_shipments_should_work() {
        let input = "2015-02-01 S MR\n".repeat(4);
        let buff = Cursor::new(input);
        let shipments = process_shipments(Box::new(buff));
        let expected = "2015-02-01 S MR 1.50 0.50\n".repeat(4);
        assert_eq!(trim_data(&expected), format!("{}", shipments));
    }

    #[test]
    fn discounts_limits_for_small_shipments_should_work() {
        let input = "2015-02-01 S MR\n".repeat(21);
        let buff = Cursor::new(input);
        let shipments = process_shipments(Box::new(buff));
        let mut expected = "2015-02-01 S MR 1.50 0.50\n".repeat(20);
        expected += "2015-02-01 S MR 2.00 -";
        assert_eq!(trim_data(&expected), format!("{}", shipments));
    }

    #[test]
    fn large_shipments_should_work() {
        let input = "2015-02-03 L LP\n".repeat(3);
        let buff = Cursor::new(input);
        let shipments = process_shipments(Box::new(buff));
        let mut expected = "2015-02-03 L LP 6.90 -\n".repeat(2);
        expected += "2015-02-03 L LP 0.00 6.90";
        assert_eq!(trim_data(&expected), format!("{}", shipments));
    }

    #[test]
    fn discount_limits_for_large_shipments_should_work() {
        let input = "2015-02-03 L LP\n".repeat(6);
        let buff = Cursor::new(input);
        let shipments = process_shipments(Box::new(buff));
        let mut expected = "2015-02-03 L LP 6.90 -\n".repeat(2);
        expected += "2015-02-03 L LP 0.00 6.90\n";
        expected += "2015-02-03 L LP 6.90 -\n".repeat(3).as_str();
        assert_eq!(trim_data(&expected), format!("{}", shipments));
    }

    #[test]
    fn mixed_discounts_should_work() {
        let input = "2015-02-03 L LP\n".repeat(6) + "2015-02-01 S MR\n";
        let buff = Cursor::new(input);
        let shipments = process_shipments(Box::new(buff));
        let mut expected = "2015-02-03 L LP 6.90 -\n".repeat(2);
        expected += "2015-02-03 L LP 0.00 6.90\n";
        expected += "2015-02-03 L LP 6.90 -\n".repeat(3).as_str();
        expected += "2015-02-01 S MR 1.50 0.50\n";
        assert_eq!(trim_data(&expected), format!("{}", shipments));
    }
}
