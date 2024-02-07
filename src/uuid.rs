use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid as uuid_parser;

#[derive(Debug, Deserialize, Serialize)]
pub struct Uuid([i32; 4]);

impl Uuid {
    pub fn new(uuid: &str) -> Self {
        let uuid = uuid_parser::from_str(uuid).unwrap();
        let num = uuid.as_u128();
        Self::from([
            (num >> 96) as i32,
            (num >> 64) as i32,
            (num >> 32) as i32,
            (num as i32),
        ])
    }

    pub(crate) fn to_u128(&self) -> u128 {
        let mut bytes = [0u8; 16];
        let (left, right) = bytes.split_at_mut(8);
        let (first, second) = left.split_at_mut(4);
        let (third, fourth) = right.split_at_mut(4);

        first.copy_from_slice(&self.0[0].to_be_bytes()[..]);
        second.copy_from_slice(&self.0[1].to_be_bytes()[..]);
        third.copy_from_slice(&self.0[2].to_be_bytes()[..]);
        fourth.copy_from_slice(&self.0[3].to_be_bytes()[..]);
        u128::from_be_bytes(bytes)
    }
}

impl From<[i32; 4]> for Uuid {
    fn from(value: [i32; 4]) -> Self {
        Self(value)
    }
}

impl ToString for Uuid {
    fn to_string(&self) -> String {
        uuid_parser::from_u128(self.to_u128()).to_string()
    }
}
