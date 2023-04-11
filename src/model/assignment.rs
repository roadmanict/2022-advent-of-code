pub struct Assignment {
    pub start: u8,
    pub end: u8,
}
impl Assignment {
    pub fn new(start: u8, end: u8) -> Self {
        Self { start, end }
    }
}
