use crate::customer::Customer;
use contracts::{requires, ensures, invariant};

pub struct Paperboy {
    first_name: String,
    last_name: String,
    customer: Option<Customer>,
}

impl Paperboy {
    pub fn new(fname: &str, lname: &str) -> Self {
        Paperboy {
            first_name: String::from(fname),
            last_name: String::from(lname),
            customer: None,
        }
    }

    pub fn get_first_name(&self) -> &str {
        &self.first_name
    }

    pub fn get_last_name(&self) -> &str {
        &self.first_name
    }

    pub fn serve(&mut self, customer: Option<Customer>) {
        if let Some(person) = customer {
            self.set_customer(person);
        }
    }

    pub fn demand(&mut self, price: f32) {
        if let Some(ref mut person) = self.customer {
            person.get_wallet().subtract(price);
        }
    }

    fn set_customer(&mut self, customer: Customer) {
        self.customer = Some(customer);
    }
}