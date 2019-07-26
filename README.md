# bitpacker
[![Build Status](https://travis-ci.org/jstasiak/bitpacker.svg?branch=master)](https://travis-ci.org/jstasiak/bitpacker)
[![Coverage Status](https://coveralls.io/repos/github/jstasiak/bitpacker/badge.svg?branch=master)](https://coveralls.io/github/jstasiak/bitpacker?branch=master)

Pack numbers tightly. In Rust.

## What is this?

I heard about this fun exercise of packing numbers tightly using as few bits as
possible for fun and profit, so I decided to see what's all that about.

This is a purely exploratory/learning project and literally nothing should be
expected of it.


## The API

```rust
let mut packer = Packer::new();

// pack() will panic if you pass bits lower than 1 or greater than 64
packer.pack(2, 2);
packer.pack(7, 4);
packer.pack(1, 1);

// In packer total_bits() will return the precise number of bits packed.
dbg!(packer.total_bits()); // Prints 7

// Prints [158]. 158 in binary is 0b10011110. In that we have:
// * 0b10 (2 bits) -> 2
// * 0b0111 (4 bits) -> 7
// * 0b1 (1 bit) -> 1
// * 0b0 (1 bit) -> unused space
dbg!(packer.buffer());

let mut unpacker = Unpacker::new(packer.buffer());
dbg!(unpacker.total_bits()); // 8, because unpacker doesn't know if there are unused bits.
dbg!(unpacker.remaining_bits()); // 8, see above + we haven't unpacked anything yet
dbg!(unpacker.unpack(2)); // 2
dbg!(unpacker.unpack(4)); // 7
dbg!(unpacker.unpack(1)); // 1
dbg!(unpacker.remaining_bits()); // 1, because there's one unused bit left. It's value is undefined.
```
