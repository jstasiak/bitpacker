struct Packer {
    _buffer: Vec<u8>,
    _byteoffset: usize,
    _bitoffset: usize,
}

impl Packer {
    pub fn new() -> Packer {
        Packer {
            _buffer: Vec::new(),
            _byteoffset: 0,
            _bitoffset: 0,
        }
    }

    pub fn buffer(&self) -> &[u8] {
        &self._buffer
    }

    pub fn byteoffset(&self) -> usize {
        self._byteoffset
    }

    pub fn bitoffset(&self) -> usize {
        self._bitoffset
    }

    pub fn pack(&mut self, value: u64, bits: usize) {
        assert!([8, 16, 24, 32, 40, 48, 56, 64].contains(&bits));
        let mut bits = bits;
        while bits > 0 {
            self._buffer.push(value as u8);
            bits -= 8;
        }
    }

    pub fn total_bits(&self) -> usize {
        self._byteoffset * 8 + self._bitoffset
    }
}

struct Unpacker<'a> {
    _buffer: &'a [u8],
    _byteoffset: usize,
    _bitoffset: usize,
}

impl<'a> Unpacker<'a> {
    pub fn new(buffer: &'a [u8]) -> Unpacker {
        Unpacker {
            _buffer: buffer,
            _byteoffset: 0,
            _bitoffset: 0,
        }
    }
    pub fn byteoffset(&self) -> usize {
        self._byteoffset
    }

    pub fn bitoffset(&self) -> usize {
        self._bitoffset
    }

    pub fn total_bits(&self) -> usize {
        self._buffer.len() * 8
    }

    pub fn remaining_bits(&self) -> usize {
        self.total_bits() - (self._byteoffset * 8 + self._bitoffset)
    }

    pub fn unpack(&mut self, bits: usize) -> u64 {
        assert!([8, 16, 24, 32, 40, 48, 56, 64].contains(&bits));
        assert!(bits < self.remaining_bits());
        let mut bits = bits;
        let mut value: u64 = 0;
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{Packer, Unpacker};

    fn verify_numbers(numbers: &[(u64, usize)]) {
        let mut packer = Packer::new();
        let mut total_bits = 0usize;
        for (index, (number, bits)) in numbers.iter().enumerate() {
            eprintln!(
                "Packing iteration {}, packing {} bits of {}",
                index, bits, number,
            );
            packer.pack(*number, *bits);
            total_bits += bits;
            assert_eq!(packer.total_bits(), total_bits);

            let mut bytes_taken = total_bits / 8;
            if total_bits % 8 > 0 {
                bytes_taken += 1;
            }
            assert_eq!(packer.buffer().len(), bytes_taken);
        }
        let mut unpacker = Unpacker::new(packer.buffer());
        for (index, (number, bits)) in numbers.iter().enumerate() {
            eprintln!("Unpacking iteration {}, reading {}-bit number", index, bits,);
            assert_eq!(unpacker.unpack(*bits), *number);
        }
    }

    #[test]
    fn it_works() {
        verify_numbers(&[]);
    }
}
