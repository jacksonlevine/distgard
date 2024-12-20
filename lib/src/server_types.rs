//use serde::{Serialize, Deserialize};

// use std::fmt::{self, Display, Formatter};

use borsh::*;

// use uuid::Uuid;
use bevy::prelude::*;


// use crate::vec;


pub const MOB_BATCH_SIZE: usize = 16;
#[derive(Clone, Debug, Copy)]
pub struct SerVec3(pub Vec3);
#[derive(Clone, Debug, Copy)]
pub struct SerIVec3(pub IVec3);

impl BorshSerialize for SerVec3 {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.0.x.serialize(writer)?;
        self.0.y.serialize(writer)?;
        self.0.z.serialize(writer)?;
        Ok(())
    }
}

impl BorshDeserialize for SerVec3 {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(SerVec3(Vec3 {
            x: BorshDeserialize::deserialize(buf)?,
            y: BorshDeserialize::deserialize(buf)?,
            z: BorshDeserialize::deserialize(buf)?,
        }))
    }
    
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        Ok(SerVec3(Vec3 {
            x: BorshDeserialize::deserialize_reader(reader)?,
            y: BorshDeserialize::deserialize_reader(reader)?,
            z: BorshDeserialize::deserialize_reader(reader)?,
        }))
    }
}

impl BorshSerialize for SerIVec3 {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.0.x.serialize(writer)?;
        self.0.y.serialize(writer)?;
        self.0.z.serialize(writer)?;
        Ok(())
    }
}

impl BorshDeserialize for SerIVec3 {
    fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        Ok(SerIVec3(IVec3 {
            x: BorshDeserialize::deserialize(buf)?,
            y: BorshDeserialize::deserialize(buf)?,
            z: BorshDeserialize::deserialize(buf)?,
        }))
    }
    
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        Ok(SerIVec3(IVec3 {
            x: BorshDeserialize::deserialize_reader(reader)?,
            y: BorshDeserialize::deserialize_reader(reader)?,
            z: BorshDeserialize::deserialize_reader(reader)?,
        }))
    }
}

#[derive(Clone, BorshDeserialize, BorshSerialize, Debug)]
pub struct WorldInfoYo {
    pub seed: u32,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize)]
pub enum Message {
    None,
    Hello(u32),
    WorldRealInfo(WorldInfoYo),
    // RequestUdm,
    // RequestSeed,
    // RequestPt,
    // ReqChestReg,
    RequestWorldInfo,

    // ChestReg(Vec<u8>), //If somethings a Vec<u8> its a db file
    // Pt(u8),
    // Udm(Vec<u8>),
    // Seed(u32),

    //ChestReg, Pt, Udm, Seed
    WorldInfo(Vec<u8>, u8, Vec<u8>, u32),

    TimeUpdate(f32),
    PlayerUpdate((u64, u64), SerVec3, SerVec3),
    BlockSet(SerIVec3, u32),
    MultiBlockSet(Vec<(SerIVec3, u32)>),
    YourId(u64, u64),
    TellYouMyID(u64, u64),
    ChestUpdate(SerIVec3, usize, (u32, u32)),
    InvUpdate(usize, (u32, u32)),
    ItemToYourMouse((u32, u32)),
    Disconnect,

    //TODO: These
    MobUpdate,
    MobUpdateBatch,
    //RequestTakeoff,
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct Message {
//     pub message_type: MessageType,
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
//     pub rot: f32,
//     pub info: u32,
//     pub info2: u32,
//     pub infof: f32,
//     pub goose: (u64, u64),
//     pub otherpos:IVec3,
//     pub bo: bool,
//     pub hostile: bool,


//     pub count: u8,
//     pub msgs: [MobMessage; MOB_BATCH_SIZE]
// }

// impl Display for Message {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(
//             f,
//             "Message {{ type: {}, x: {}, y: {}, z: {}, rot: {}, info: {}, info2: {} }}",
//             self.message_type, self.x, self.y, self.z, self.rot, self.info, self.info2
//         )
//     }
// }

// impl Display for MobUpdateBatch {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "MobUpdateBatch {{ count: {}, msgs: [", self.count)?;
//         for (i, msg) in self.msgs.iter().enumerate() {
//             if i != 0 {
//                 write!(f, ", ")?;
//             }
//             write!(f, "{}", msg)?;
//         }
//         write!(f, "] }}")
//     }
// }



// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Copy)]
// pub enum MessageType {
//     None,
//     RequestUdm,
//     RequestSeed,
//     RequestPt,
//     Pt,
//     Udm,
//     Seed,
//     PlayerUpdate,
//     BlockSet,
//     RequestTakeoff,
//     YourId,
//     MobUpdate,
//     NewMob,
//     WhatsThatMob,
//     ShutUpMobMsgs,
//     MobUpdateBatch,
//     TimeUpdate,
//     TellYouMyID,
//     MultiBlockSet,
//     ChestReg,
//     ReqChestReg,
//     ChestInvUpdate,
//     Disconnect
// }

// impl Display for MessageType {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         match self {
//             MessageType::RequestUdm => {
//                 write!(f, "RequestUdm")
//             },
//             MessageType::RequestSeed => {
//                 write!(f, "RequestSeed")
//             },
//             MessageType::PlayerUpdate => {
//                 write!(f, "PlayerUpdate")
//             },
//             MessageType::BlockSet => {
//                 write!(f, "BlockSet")
//             },
//             MessageType::Udm => {
//                 write!(f, "Udm")
//             },
//             MessageType::Seed => {
//                 write!(f, "Seed")
//             },
//             MessageType::RequestTakeoff => {
//                 write!(f, "RequestTakeoff")
//             },
//             MessageType::RequestPt => {
//                 write!(f, "RequestPt")
//             },
//             MessageType::Pt => {
//                 write!(f, "Pt")
//             },
//             MessageType::None => {
//                 write!(f, "None")
//             },
//             MessageType::YourId => {
//                 write!(f, "YourId")
//             },
//             MessageType::MobUpdate => {
//                 write!(f, "MobUpdate")
//             },
//             MessageType::NewMob => {
//                 write!(f, "NewMob")
//             },
//             MessageType::WhatsThatMob => {
//                 write!(f, "WhatsThatMob")
//             },
//             MessageType::ShutUpMobMsgs => {
//                 write!(f, "ShutUpMobMsgs")
//             },
//             MessageType::MobUpdateBatch => {
//                 write!(f, "MobUpdateBatch")
//             },
//             MessageType::TimeUpdate => {
//                 write!(f, "TimeUpdate")
//             },
//             MessageType::TellYouMyID => {
//                 write!(f, "Tellyoumyid")
//             },
//             MessageType::MultiBlockSet => {
//                 write!(f, "MultiBlockSet")
//             },
//             MessageType::ChestReg => {
//                 write!(f, "ChestReg")
//             },
//             MessageType::ReqChestReg => {
//                 write!(f, "ReqChestReg")
//             },



//             /*OTHERPOS: CURRENT CHEST */
//             /*INFO: DEST SLOT INDEX */
//             /*INFO2: SLOT INDEX TYPE */
//             /*ROT: ID */
//             /*INFOF: COUNT */
//             /*X, Y:   SLOT MOVED TO MOUSE OF <GOOSE> PLAYER */
//             /*Z: IF MOUSE_SLOT IS REPLACED */

//             /*CHEST = 0; INV = 1; NONE = 2 */
//             MessageType::ChestInvUpdate => {
//                 write!(f, "ChestInvUpdate")
//             },
//             MessageType::Disconnect => {
//                 write!(f, "Disconnect")
//             }
//         }
//     } 
// }

// impl Message {

//     #[inline]
//     #[must_use]
//     pub const fn from_mob_message(message: &MobMessage) -> Self {
//         Self {
//             message_type: message.message_type, x:message.x, y: message.y, z: message.z, rot: message.rot, info: message.info, info2: message.info2, infof: message.infof, goose: message.goose, otherpos: message.otherpos, bo: message.bo, hostile: message.hostile,
//             count: 0, msgs: [MobMessage::EMPTY; MOB_BATCH_SIZE]
//         }
//     }

//     pub fn inoculate_with_mobupdates(&mut self, count: usize, slice: &[Message]) {
//         if count > MOB_BATCH_SIZE {
//             panic!("No MobUpdateBatch over size {}", MOB_BATCH_SIZE);
//         }
//         let _emptymsg = Message::new(MessageType::None, Vec3::ZERO, 0.0, 0);

//         for i in 0..count {
//             self.msgs[i] = MobMessage::from_message(&slice[i]);
//         }
//         self.count = count as u8;
//         // for i in count..self.msgs.len() {
//         //     self.msgs[i] = MobMessage::from_message(&emptymsg);
//         // }

//     }

//     pub fn invupdate(slot: usize, newid: u32, newamount: u32) -> Message {

//         let mut msg = Message::new(MessageType::ChestInvUpdate, Vec3::ZERO, newid as f32, slot as u32);
//         msg.infof = newamount as f32;
//         msg.info2 = 1;
//         msg
//     }

    
//     pub fn new(t: MessageType, pos: Vec3, rot: f32, info: u32) -> Message {
//         Message {
//             message_type: t,
//             x: pos.x,
//             y: pos.y,
//             z: pos.z,
//             rot,
//             info,
//             info2: 0,
//             infof: 1.0,
//             goose: Uuid::new_v4().as_u64_pair(),
//             otherpos:IVec3::new(0,0,0),
//             bo: false,
//             hostile: false,

//             count: 0,
//             msgs: [MobMessage::EMPTY; MOB_BATCH_SIZE]
//         }
//     }

//     pub fn get_serialized_size() -> usize {
//         let m = Message::new(MessageType::BlockSet, Vec3::new(0.0,0.0,0.0), 0.0, 0);
//         bincode::serialized_size(&m).unwrap() as usize
//     }
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct MobMessage {
//     pub message_type: MessageType,
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
//     pub rot: f32,
//     pub info: u32,
//     pub info2: u32,
//     pub infof: f32,
//     pub goose: (u64, u64),
//     pub otherpos:IVec3,
//     pub bo: bool,
//     pub hostile: bool,
// }

// impl MobMessage {
//     pub const EMPTY: Self = Self::new();

//     #[inline]
//     #[must_use]
//     pub const fn new() -> Self {
//         Self { message_type: MessageType::None, x:0.0, y: 0.0, z: 0.0, rot: 0.0, info: 0, info2: 0, infof: 0.0, goose: (0, 0), otherpos:IVec3{x:0, y:0, z:0}, bo: false, hostile: false}
//     }

//     #[inline]
//     #[must_use]
//     pub const fn from_message(message: &Message) -> Self {
//         Self {
//             message_type: message.message_type, x:message.x, y: message.y, z: message.z, rot: message.rot, info: message.info, info2: message.info2, infof: message.infof, goose: message.goose, otherpos: message.otherpos, bo: message.bo, hostile: message.hostile
//         }
//     }

// }




// #[derive(Serialize, Deserialize)]
// pub struct Entry {
//     pub key:IVec3,
//     pub value: u32,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct MobUpdateBatch {
//     pub count: u8,
//     pub msgs: [Message; MOB_BATCH_SIZE]
// }

// impl MobUpdateBatch {
//     pub fn new(count: usize, slice: &[Message]) -> MobUpdateBatch {
//         if count > MOB_BATCH_SIZE {
//             panic!("No MobUpdateBatch over size {}", MOB_BATCH_SIZE);
//         }
//         let emptymsg = Message::new(MessageType::None, Vec3::ZERO, 0.0, 0);


//         let mut msgs: [Message; MOB_BATCH_SIZE] = [
//             emptymsg.clone(),
//             emptymsg.clone(),
//             emptymsg.clone(),
//             emptymsg.clone(),

//             emptymsg.clone(),
//             emptymsg.clone(),
//             emptymsg.clone(),
//             emptymsg.clone(),


//             emptymsg.clone(),
//             emptymsg.clone(),
//             emptymsg.clone(),
//             emptymsg.clone(),

//             emptymsg.clone(),
//             emptymsg.clone(),
//             emptymsg.clone(),
//             emptymsg.clone(),
//         ];

//         for i in 0..count {
//             msgs[i] = slice[i].clone();
//         }

//         MobUpdateBatch {
//             count: count as u8,
//             msgs
//         }
//     }
// }


