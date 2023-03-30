pub struct Food {
    pub calories: u32,
}

impl Food {
    pub fn new(calories: u32) -> Self {
        Self { calories }
    }
}
