use crate::wallet::Wallet;

#[derive(Clone)]
pub struct Customer {
    first_name: String,
    last_name: String,
    wallet: Wallet,
}

impl Customer {
    pub fn new(fname: &str, lname: &str, balance: f32) -> Self {
        Customer {
            first_name: String::from(fname),
            last_name: String::from(lname),
            wallet: Wallet::new(balance),
        }
    }

    pub fn get_first_name(&self) -> &str {
        &self.first_name
    }

    pub fn get_last_name(&self) -> &str {
        &self.last_name
    }

    //pub fn get_wallet(&mut self) -> &mut Wallet {
    //    &mut self.wallet
    //}

    pub fn payment(&mut self, price: f32) -> bool {
        if price <= self.wallet.get_balance() {
            self.wallet.subtract(price);
            true
        } else {
            false
        }
    }
}