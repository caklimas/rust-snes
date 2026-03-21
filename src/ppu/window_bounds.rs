#[derive(Clone, Copy, Default)]
pub struct WindowBounds {
    pub left: u8,
    pub right: u8,
}

impl WindowBounds {
    pub fn is_active(&self, x: u8, mode: u8) -> Option<bool> {
        match mode {
            0 => None,
            1 => Some(x >= self.left && x <= self.right),
            2 => Some(x < self.left || x > self.right),
            _ => unimplemented!(),
        }
    }
}
