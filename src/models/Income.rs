pub struct Income {
    name: String,
    amount: f32,
}

impl Income {
    pub fn new(name: String, amount: f32) -> Income {
        Income {
            name: name,
            amount: amount,
        }
    }
    pub fn get_amount(&self) -> f32 {
        self.amount
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name
    }
}
