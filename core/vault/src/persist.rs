//! Background process for writing shit.
//! 
//! //Commented things out because criterion is bitching about this even though its unused. Will someone tell criterion to GET OFF MY BACK PLEASE?

use borsh::{BorshDeserialize as De, BorshSerialize as Ser};
use std::sync::mpsc::{Receiver, Sender};
use std::fs::File;
use std::io;
use core::ptr::NonNull;
use smallvec::SmallVec;

type KeyRepr = SmallVec<[u8; 32]>;
type Body = SmallVec<[u8; 2048]>;

#[must_use]
#[repr(transparent)]
pub struct Key {
    key: KeyRepr
}

// impl Key {
//     #[inline]
//     pub fn new<K: AsRef<u8>>(key: K) -> Self {
//         Self { key: KeyRepr::from_slice(key.as_ref()) }
//     }
// }

pub enum Event {
    Write(Body),
    Read,
    Delete,
}

#[must_use]
pub struct Message {
    key: Key,
    event: Event
}

// impl Message {
//     #[inline]
//     pub fn write<T: Ser>(key: Key, data: &T) -> io::Result<Self> {
//         let mut body = Body::new();
//         borsh::to_writer(&mut info, data)
//             .map(move |()| Self { key, event: Event::Write(body) })
//     }
// }

#[must_use]
#[derive(Clone)]
pub struct Buddy {
    send: Sender<Message>
}

impl Buddy {
    #[inline]
    const fn new(send: Sender<Message>) -> Self {
        Self { send }
    }

    //const fn store
}

#[repr(transparent)]
pub struct Persist {
    recv: Receiver<Message>
}

impl Persist {
    
}
