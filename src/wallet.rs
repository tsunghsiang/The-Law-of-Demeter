use contracts::{requires, ensures, invariant};

pub struct Wallet {
    balance: f32,
}

impl Wallet {
    #[requires(value >= 0.00, "Wallet balance should be greater than 0.00")]
    #[ensures(ret.get_balance() == value, "Wallet balance should be equivalent to initialized value")]
    pub fn new(value: f32) -> Self {
        Wallet {
            balance: value,
        }
    }

    #[requires(self.balance >= 0.00, "Wallet balance should be greater than 0.00")]
    #[ensures(ret == self.balance, "Inner balance should be equivalent to the getter result")]
    #[invariant(self.balance >= 0.00, "Wallet balance should be greater than 0.00")]
    pub fn get_balance(&self) -> f32 {
        self.balance
    }

    #[requires(value >= 0.00, "Added value should be greater than 0.00")]
    #[requires(self.balance >= 0.00, "Wallet balance should be greater than 0.00")]
    #[ensures(self.balance >= 0.00, "Wallet balance should be greater than 0.00")]
    #[invariant(self.balance >= 0.00, "Wallet balance should be greater than 0.00")]
    pub fn add(&mut self, value: f32) {
        self.balance += value;
    }

    #[requires(value >= 0.00, "Subtracted value should be greater than 0.00")]
    #[requires(self.balance >= value, "Value to be subtracted should not be greater than the wallet balance")]
    #[ensures(self.balance >= 0.00, "Wallet balance should be greater than 0.00")]
    #[invariant(self.balance >= 0.00, "Wallet balance should be greater than 0.00")]    
    pub fn subtract(&mut self, value: f32) {
        if self.get_balance() >= value {
            self.balance -= value;
        }
    }
}