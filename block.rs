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
    pub fn get_from_id(&self) -> Color{
        let mut (r,g,b) = (0.0,0.0,0.0);
        match self.material => {
            0 => (),
            1 => {r = 0.33;g = 0.49;b = 0.27;},
            2 => {r = 0.6; g = 0.6;b = 0.6;}
            _ => {}
        }
        Color::rgb(r,g,b).into()
    }
}

