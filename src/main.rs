mod wallet;
mod customer;
 
use wallet::Wallet;
use customer::Customer;

fn main() {
    let customer = Customer::new("Andy", "Wu", 1000.00);
}
