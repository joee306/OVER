use crate::block::*;

use bevy::prelude::*;

const CHUNK_DIMENSIONS : (usize,usize,usize) = (16,128,16); //xyz

#[derive(Component)]
pub struct Chunk {
    pub coords : (i64, i64, i64),
    pub data : [[[Block ; CHUNK_DIMENSIONS.0] ; CHUNK_DIMENSIONS.1] ; CHUNK_DIMENSIONS.2],  
    pub loaded : bool,
}

impl Chunk {
    pub fn new(cx : i64,cy : i64,cz : i64) -> Self {
        Self {
            coords : (cx,0,cz),
            data :
            [[[ Block {
                render_side : LayerSide { XM : false , XP : false , YM : false , YP : false , ZM : false , ZP : false },
                material : 1,
            }; CHUNK_DIMENSIONS.0] ; CHUNK_DIMENSIONS.1] ; CHUNK_DIMENSIONS.2],
            loaded : false,
        }
    }
    pub fn get_save(&self,x : usize, y : usize, z : usize) -> Option<Block> {
        if x >= CHUNK_DIMENSIONS.0 || y >= CHUNK_DIMENSIONS.1 || z >= CHUNK_DIMENSIONS.2 {
            return None;
        } else {
            return Some(self.data[x][y][z]);
        }  
    } 
    pub fn set_save(&mut self,x : usize, y : usize, z : usize,  material : u32) { 
        if x >= CHUNK_DIMENSIONS.0 || y >= CHUNK_DIMENSIONS.1 || z >= CHUNK_DIMENSIONS.2 {
            return;
        } else {
            self.data[x][y][z].material = material;
        }
    }
    
    pub fn calc_layers(&mut self) {
        for x in 0..self.data.len() {
            for y in 0..self.data[0].len() {
                for z in 0..self.data[0][0].len() {
                    if self.data[x][y][z].material != 0 {
                        // check for block in xpos //
                        if x + 1 >= self.data.len() {
                            self.data[x][y][z].render_side.XP = true;
                        } else if self.data[x + 1][y][z].material == 0 {
                            self.data[x][y][z].render_side.XP = true;
                        }
                        /////////////////////////////
                        // check for block in xmin //
                        if x <= 0 {
                            self.data[x][y][z].render_side.XM = true;
                        } else if self.data[x - 1][y][z].material == 0 {
                            self.data[x][y][z].render_side.XM = true;
                        }
                        /////////////////////////////
                        // check for block in ypos //
                        if y + 1 >= self.data[0].len() {
                            self.data[x][y][z].render_side.YP = true;
                        } else if self.data[x][y + 1][z].material == 0 {
                            self.data[x][y][z].render_side.YP = true;
                        }
                        /////////////////////////////
                        // check for block in ymin //
                        if y <= 0 {
                            self.data[x][y][z].render_side.YM = true;
                        } else if self.data[x][y - 1][z].material == 0 {
                            self.data[x][y][z].render_side.YM = true;
                        }
                        /////////////////////////////
                        // check for block in zpos //
                        if z + 1 >= self.data[0][0].len() {
                            self.data[x][y][z].render_side.ZP = true;
                        } else if self.data[x][y][z + 1].material == 0 {
                            self.data[x][y][z].render_side.ZP = true;
                        }
                        /////////////////////////////
                        // check for block in zmin //
                        if z <= 0 {
                            self.data[x][y][z].render_side.ZM = true;
                        } else if self.data[x][y][z - 1].material == 0 {
                            self.data[x][y][z].render_side.ZM = true;
                        }
                        /////////////////////////////
                    }
                }
            }
        }
    }
}
