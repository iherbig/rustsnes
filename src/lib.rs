#![feature(core)]
#![feature(core_intrinsics)]

extern crate core;

pub mod cpu;
mod memory;
mod flags;

#[test]
fn test_memory() {
    let mut mem = memory::Memory::new();

    for i in 1..11 {
        mem.set_byte(i, i as u8);
    }

    for i in 1..11 {
        assert_eq!(i, mem.get_byte(i) as usize);
    }

    assert_eq!(0, mem.get_byte(12) as usize);
}
