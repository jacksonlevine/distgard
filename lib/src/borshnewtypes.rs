




use bevy::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};




pub struct BorshIVec3(pub IVec3);


impl BorshSerialize for BorshIVec3 {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.0.x.serialize(writer).unwrap();
        self.0.y.serialize(writer).unwrap();
        self.0.z.serialize(writer).unwrap();
        Ok(())
    }
}

impl BorshDeserialize for BorshIVec3 {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let x = i32::deserialize_reader(reader).unwrap();
        let y = i32::deserialize_reader(reader).unwrap();
        let z = i32::deserialize_reader(reader).unwrap();
        Ok(BorshIVec3(IVec3 { x, y, z }))
    }
}