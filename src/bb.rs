use crate::vec::Vec3I;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BoundingBox(#[serde(serialize_with = "nbt::int_array")] [i32; 6]);

impl BoundingBox {
    pub fn new(min_pos: Vec3I, max_pos: Vec3I) -> Self {
        Self([
            min_pos.x(),
            min_pos.y(),
            min_pos.z(),
            max_pos.x(),
            max_pos.y(),
            max_pos.z(),
        ])
    }

    pub fn min_pos(&self) -> Vec3I {
        Vec3I::new(self.0[0], self.0[1], self.0[2])
    }

    pub fn max_pos(&self) -> Vec3I {
        Vec3I::new(self.0[3], self.0[4], self.0[5])
    }
}
