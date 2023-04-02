pub struct Food {
    pub calories: u16,
}

impl Food {
    pub fn new(calories: u16) -> Self {
        Self { calories }
    }
}
