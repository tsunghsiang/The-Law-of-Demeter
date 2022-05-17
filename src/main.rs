mod wallet;
mod customer;
mod paperboy;

use customer::Customer;
use paperboy::Paperboy;

fn main() {
    let customer = Customer::new("Andy", "Wu", 1000.00);
    let mut paperboy = Paperboy::new("Jacky", "Chen");
    paperboy.serve(Some(customer));
    paperboy.demand(20.00);
}
