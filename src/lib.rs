use std::cmp;

fn bin(byte: u8) -> String {
    format!("{:#010b}", byte)
}

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

    pub fn pack(&mut self, value: u64, bits: usize) {
        assert!(bits > 0 && bits <= 64, "Invalid number of bits: {}", bits);
        let mut bits_to_write = bits;
        let mut value = value;
        while bits_to_write > 0 {
            let free_bits_left_this_byte = 8 - self._bitoffset;
            let bits_to_write_this_byte = cmp::min(free_bits_left_this_byte, bits);
            let mut byte = if self._bitoffset > 0 {
                self._buffer[self._byteoffset]
            } else {
                self._buffer.push(0);
                0
            };
            let source_bitmask = 1 << bits_to_write_this_byte - 1;
            dbg!(bin(source_bitmask));
            let source_chunk = value as u8 & source_bitmask;
            byte += source_chunk << (8 - self._bitoffset - bits_to_write_this_byte);
            dbg!(bin(byte));
            self._buffer[self._byteoffset] = byte;
            bits_to_write -= bits_to_write_this_byte;
            value = value >> bits_to_write_this_byte;
            self._bitoffset += bits_to_write_this_byte;
            if self._bitoffset == 8 {
                self._bitoffset = 0;
                self._byteoffset += 1;
            }
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

    pub fn total_bits(&self) -> usize {
        self._buffer.len() * 8
    }

    pub fn remaining_bits(&self) -> usize {
        self.total_bits() - (self._byteoffset * 8 + self._bitoffset)
    }

    pub fn unpack(&mut self, bits: usize) -> u64 {
        assert!(bits > 0 && bits <= 64, "Invalid number of bits: {}", bits);
        assert!(bits <= self.remaining_bits());
        let mut bits_to_read = bits;
        let mut value: u64 = 0;
        let mut factor = 0;
        while bits_to_read > 0 {
            let byte = self._buffer[self._byteoffset];
            let bits_available_this_byte = 8 - self._bitoffset;
            let bits_to_read_this_byte = cmp::min(bits_to_read, bits_available_this_byte);
            let remaining_bits = 8 - self._bitoffset - bits_to_read_this_byte;
            let source_bitmask = (1 << bits_to_read_this_byte - 1) << remaining_bits;
            value += ((byte & source_bitmask) as u64) >> remaining_bits << factor;
            factor += bits_to_read_this_byte;
            self._bitoffset += bits_to_read_this_byte;
            if self._bitoffset == 8 {
                self._bitoffset = 0;
                self._byteoffset += 1;
            }
            bits_to_read -= bits_to_read_this_byte;
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::{Packer, Unpacker};

    fn verify_numbers(numbers: &[(u64, usize)]) {
        eprintln!("Verifying numbers: {:?}", numbers);
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
        verify_numbers(&[
            (1 << 6 + 1, 8),
            (1 << 14 + 1, 16),
            (1 << 22 + 1, 24),
            (1 << 30 + 1, 32),
            (1 << 38 + 1, 40),
            (1 << 46 + 1, 48),
            (1 << 54 + 1, 56),
            (1 << 62 + 1, 64),
        ]);

        verify_numbers(&[(0, 1), (1, 1), (2, 2)]);
    }
}
