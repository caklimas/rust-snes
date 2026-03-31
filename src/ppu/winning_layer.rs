#[derive(Clone, Copy)]
pub struct WinningLayer {
    pub cgram_index: u8,
    pub layer: Layer,
}

#[derive(Clone, Copy, Debug)]
pub enum Layer {
    Bg1,
    Bg2,
    Bg3,
    Bg4,
    Obj,
}
