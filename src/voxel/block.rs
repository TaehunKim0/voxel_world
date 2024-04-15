use bevy::prelude::*;

pub enum BlockType {
    Water,
    Sand,
    Grass,
    Dirt,
    Stone
}

#[derive(Component)]
pub struct Block {
    block_type: BlockType,
}


