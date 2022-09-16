use bevy::prelude::*;

#[derive(Clone,Copy,Debug)]
pub struct LayerSide {
    pub XM : bool,
    pub XP : bool,
    pub YM : bool,
    pub YP : bool,
    pub ZM : bool,
    pub ZP : bool,
}

#[derive(Clone,Copy,Debug)]
pub struct Block {
    pub render_side  : LayerSide,
    pub material : u32,
}

impl Block {
    pub fn new(id : u32) -> Self {
        Self {
            render_side : LayerSide { XM : false , XP : false , YM : false , YP : false , ZM : false , ZP : false },
            material : id,
        }
    }
}

