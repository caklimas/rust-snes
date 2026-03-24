use crate::ppu::{
    bg_mode::BgMode,
    bg_sample::BgSample,
    obj_sample::ObjSample,
    winning_layer::{Layer, WinningLayer},
};

pub struct PriorityResolver {
    bg1_sample: Option<BgSample>,
    bg2_sample: Option<BgSample>,
    bg3_sample: Option<BgSample>,
    bg4_sample: Option<BgSample>,
    obj_sample: Option<ObjSample>,
}

impl PriorityResolver {
    pub fn new(
        bg1_sample: Option<BgSample>,
        bg2_sample: Option<BgSample>,
        bg3_sample: Option<BgSample>,
        bg4_sample: Option<BgSample>,
        obj_sample: Option<ObjSample>,
    ) -> Self {
        Self {
            bg1_sample,
            bg2_sample,
            bg3_sample,
            bg4_sample,
            obj_sample,
        }
    }

    pub fn get_sample(&self, bg_mode: BgMode) -> Option<WinningLayer> {
        match bg_mode.bg_mode() {
            0 => self.mode_0_sample(),
            1 => self.mode_1_sample(bg_mode.bg_priority_boost()),
            2..=5 => self.mode_2345_sample(),
            6 => self.mode_6_sample(),
            _ => None,
        }
    }

    fn mode_0_sample(&self) -> Option<WinningLayer> {
        if let Some(ObjSample {
            cg_ram_index,
            priority: 3,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: true,
        }) = self.bg1_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg1,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: true,
        }) = self.bg2_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg2,
            })
        } else if let Some(ObjSample {
            cg_ram_index,
            priority: 2,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: false,
        }) = self.bg1_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg1,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: false,
        }) = self.bg2_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg2,
            })
        } else if let Some(ObjSample {
            cg_ram_index,
            priority: 1,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: true,
        }) = self.bg3_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg3,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: true,
        }) = self.bg4_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg4,
            })
        } else if let Some(ObjSample {
            cg_ram_index,
            priority: 0,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: false,
        }) = self.bg3_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg3,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: false,
        }) = self.bg4_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg4,
            })
        } else {
            None
        }
    }

    fn mode_1_sample(&self, bg_priority_boost: bool) -> Option<WinningLayer> {
        if bg_priority_boost
            && let Some(BgSample {
                cg_ram_index,
                priority: true,
            }) = self.bg3_sample
        {
            return Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg3,
            });
        }

        if let Some(ObjSample {
            cg_ram_index,
            priority: 3,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: true,
        }) = self.bg1_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg1,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: true,
        }) = self.bg2_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg2,
            })
        } else if let Some(ObjSample {
            cg_ram_index,
            priority: 2,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: false,
        }) = self.bg1_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg1,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: false,
        }) = self.bg2_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg2,
            })
        } else if let Some(ObjSample {
            cg_ram_index,
            priority: 1,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: true,
        }) = self.bg3_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg3,
            })
        } else if let Some(ObjSample {
            cg_ram_index,
            priority: 0,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: false,
        }) = self.bg3_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg3,
            })
        } else {
            None
        }
    }

    fn mode_2345_sample(&self) -> Option<WinningLayer> {
        if let Some(ObjSample {
            cg_ram_index,
            priority: 3,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: true,
        }) = self.bg1_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg1,
            })
        } else if let Some(ObjSample {
            cg_ram_index,
            priority: 2,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: true,
        }) = self.bg2_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg2,
            })
        } else if let Some(ObjSample {
            cg_ram_index,
            priority: 1,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: false,
        }) = self.bg1_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg1,
            })
        } else if let Some(ObjSample {
            cg_ram_index,
            priority: 0,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: false,
        }) = self.bg2_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg2,
            })
        } else {
            None
        }
    }

    fn mode_6_sample(&self) -> Option<WinningLayer> {
        if let Some(ObjSample {
            cg_ram_index,
            priority: 3,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: true,
        }) = self.bg1_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg1,
            })
        } else if let Some(ObjSample {
            cg_ram_index,
            priority: 2,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(ObjSample {
            cg_ram_index,
            priority: 1,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else if let Some(BgSample {
            cg_ram_index,
            priority: false,
        }) = self.bg1_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Bg1,
            })
        } else if let Some(ObjSample {
            cg_ram_index,
            priority: 0,
        }) = self.obj_sample
        {
            Some(WinningLayer {
                cgram_index: cg_ram_index,
                layer: Layer::Obj,
            })
        } else {
            None
        }
    }
}
