mod wallet;
mod customer;
mod paperboy;

use customer::Customer;
use paperboy::Paperboy;

fn main() {
    let customer = Customer::new("Andy", "Wu", 1000.00);
    let mut paperboy = Paperboy::new("Jacky", "Chen");
    // Serve a customer
    paperboy.serve(Some(customer));
    // Demand payment
    paperboy.demand(20.00);
}
