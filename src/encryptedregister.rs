use core::array;
use tfhe::prelude::*;
use tfhe::{ClientKey, FheUint64, FheUint8};

use crate::register::Register;

const DATA_SIZE: usize = 16;

pub struct EncryptedRegister {
    pub peer: FheUint64,
    pub clock: FheUint64,
    pub value: [FheUint8; DATA_SIZE],
}

impl EncryptedRegister {
    pub fn new(peer: u64) -> Register {
        Register {
            peer,
            clock: 0,
            value: [0; DATA_SIZE],
        }
    }
    pub fn encrypt(clear: &Register, key: &ClientKey) -> EncryptedRegister {
        EncryptedRegister {
            peer: FheUint64::encrypt(clear.peer, key),
            clock: FheUint64::encrypt(clear.clock, key),
            value: array::from_fn(|i| FheUint8::encrypt(clear.value[i], key)),
        }
    }

    pub fn decrypt(&self, key: &ClientKey) -> Register {
        Register {
            peer: FheUint64::decrypt(&self.peer, key),
            clock: FheUint64::decrypt(&self.clock, key),
            value: array::from_fn(|i| FheUint8::decrypt(&self.value[i], key)),
        }
    }

    pub fn merge(&mut self, other: &EncryptedRegister) {
        let higher_clock = self.clock.gt(&other.clock);

        let equal_clock = self.clock.eq(&other.clock);
        let higher_peer = self.peer.gt(&other.peer);

        let keep_self = higher_clock | (equal_clock & higher_peer);

        self.peer = keep_self.select(&self.peer, &other.peer);
        self.clock = keep_self.select(&self.clock, &other.clock);
        self.value = array::from_fn(|i| keep_self.select(&self.value[i], &other.value[i]));
    }
}