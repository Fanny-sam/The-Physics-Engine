use crate::constants::*
#[inline(always)]
impl Matrix2 {
    pub fn is_zero(&self) -> bool {
        self.col1.x == 0.0 &&
        self.col1.y == 0.0 &&
        self.col2.x == 0.0 &&
        self.col2.y == 0.0
    }
}
pub struct Batch {
    pub values: [Matrix2; 4096],
}