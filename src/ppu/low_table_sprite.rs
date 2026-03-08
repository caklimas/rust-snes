use crate::ppu::packed_attributes::PackedAttributes;

#[derive(Default)]
pub struct LowTableSprite {
    pub x: u8,
    pub y: u8,
    pub tile_number: u8,
    pub packed_attributes: PackedAttributes,
}
