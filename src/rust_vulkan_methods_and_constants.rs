#[inline(always)]
pub fn is_zero(&self) -> bool {
    self.col1.x == 0.0 &&
    self.col1.y == 0.0 &&
    self.col2.x == 0.0 &&
    self.col2.y == 0.0
}
struct Batch {
    values: [Matrix2; 4096],
}