use std::fmt::{self, Display, Formatter};
use std::hash::Hash;
use std::ops::{Add, Sub};
use std::str::FromStr;

use bevy::math::Vec3;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy,  BorshSerialize, BorshDeserialize)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

impl Add for IVec2 {
    type Output = IVec2;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        IVec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for IVec2 {
    type Output = IVec2;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        IVec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl FromStr for IVec3 {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, ' ');
        if let (Some(x), Some(y), Some(z)) = (parts.next(), parts.next(), parts.next()) {
            Ok(IVec3::new(
                i32::from_str(x).unwrap(),
                i32::from_str(y).unwrap(),
                i32::from_str(z).unwrap()))
        } else {
            Err(String::from("Error"))
        }
    }
    
    type Err = String;
}

impl Display for IVec3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    } 
}

impl Add for IVec3 {
    type Output = IVec3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        IVec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for IVec3 {
    type Output = IVec3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        IVec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl IVec3 {
    pub const fn new(x: i32, y: i32, z: i32) -> IVec3 {
        IVec3 {
            x,
            y,
            z
        }
    }
    pub fn as_vec3(&self) -> Vec3 {

        Vec3::new(self.x as f32, self.y as f32, self.z as f32)

    }
}

impl IVec2 {
    pub const fn new(x: i32, y: i32) -> IVec2 {
        IVec2 {
            x,
            y
        }
    }
}
