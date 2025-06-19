const DATA_SIZE: usize = 16;

pub struct Register {
    pub peer: u64,
    pub clock: u64,
    pub value: [u8; DATA_SIZE],
}

impl Register {
    pub fn new(peer: u64) -> Register {
        Register {
            peer,
            clock: 0,
            value: [0; DATA_SIZE],
        }
    }

    pub fn set(&mut self, peer: u64, value: [u8; DATA_SIZE]) {
        self.peer = peer;
        self.clock += 1;
        self.value = value;
    }

    pub fn set_string(&mut self, peer: u64, value: &str) {
        let bytes = value.as_bytes();
        let len = bytes.len().min(DATA_SIZE);

        let mut data = [0; DATA_SIZE];
        data[..len].copy_from_slice(&bytes[..len]);

        self.set(peer, data);
    }

    pub fn merge(&mut self, other: &Register) {
        // If local clock is ahead of other clock, just ignore the update
        if self.clock > other.clock {
            return;
        };

        // if clocks are equal - tiebreak based on the peer id
        if self.clock == other.clock && self.peer > other.peer {
            return;
        }

        // Set this Register to be `other`
        self.peer = other.peer;
        self.clock = other.clock;
        self.value = other.value;
    }
}