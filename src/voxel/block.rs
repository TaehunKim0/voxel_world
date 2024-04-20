use std::vec;

pub struct BlockType {
    pub block_name: String,
    pub is_solid: bool,
    pub back_face_texture: i32,
    pub front_face_texture: i32,
    pub top_face_texture: i32,
    pub bottom_face_texture: i32,
    pub right_face_texture: i32,
    pub left_face_texture: i32,
}

impl BlockType {
    pub fn get_texture_id(&mut self, face_index: i32) -> i32 {
        match face_index {
            0 => self.back_face_texture,
            1 => self.front_face_texture,
            2 => self.top_face_texture,
            3 => self.bottom_face_texture,
            4 => self.left_face_texture,
            5 => self.right_face_texture,
            _ => self.back_face_texture,
        }
    }

    pub fn new_grass() -> BlockType {
        BlockType {
            block_name: "Grass".to_string(),
            is_solid: true,
            back_face_texture: 14,
            front_face_texture: 14,
            top_face_texture: 11,
            bottom_face_texture: 13,
            right_face_texture: 14,
            left_face_texture: 14,
        }
    }

    pub fn new_stone() -> BlockType {
        BlockType {
            block_name: "Stone".to_string(),
            is_solid: true,
            back_face_texture: 12,
            front_face_texture: 12,
            top_face_texture: 12,
            bottom_face_texture: 12,
            right_face_texture: 12,
            left_face_texture: 12,
        }
    }
    pub fn new_bed_rock() -> BlockType {
        BlockType {
            block_name: "BedRock".to_string(),
            is_solid: true,
            back_face_texture: 5,
            front_face_texture: 5,
            top_face_texture: 5,
            bottom_face_texture: 5,
            right_face_texture: 5,
            left_face_texture: 5,
        }
    }

    pub fn new_sand() -> BlockType {
        BlockType {
            block_name: "BedRock".to_string(),
            is_solid: true,
            back_face_texture: 6,
            front_face_texture: 6,
            top_face_texture: 6,
            bottom_face_texture: 6,
            right_face_texture: 6,
            left_face_texture: 6,
        }
    }
}

pub struct Block {
    pub block_types: Vec<BlockType>,
}

impl Block {
    pub fn new() -> Block {
        let block_types = vec![
            BlockType::new_grass(),
            BlockType::new_stone(),
            BlockType::new_bed_rock(),
            BlockType::new_sand(),
        ];

        Block { block_types }
    }
}
