#[allow(unused_imports)] use std::alloc::{alloc, dealloc, Layout};

// The Machine
#[allow(dead_code)]
pub const GENERAL_PURPOSE_REGISTER_COUNT: u32 = 256;
pub const MEMORY_SIZE: u32 = MAX_TTRA as u32; // 4 GB of main memory

pub type Register = u64;

#[allow(dead_code)]
pub struct MmixMachine {
    pub general_purpose_registers: [Register; GENERAL_PURPOSE_REGISTER_COUNT as usize],
    pub special_purpose_registers: [Register; 32],
    pub memory: *mut u8 // [0; usize::MAX] is too big for the architecture
}

impl MmixMachine {
    pub fn new() -> Self {
        let layout = Layout::new::<[u8; MAX_TTRA as usize]>();

        unsafe {
            Self {
                general_purpose_registers: [0; GENERAL_PURPOSE_REGISTER_COUNT as usize],
                special_purpose_registers: [0; 32],
                memory: alloc(layout) // [0; usize::MAX] is too big for the architecture
            }
        }
    }
}

impl Drop for MmixMachine {
    fn drop(&mut self) {
        unsafe { dealloc(self.memory, Layout::new::<[u8; MAX_TTRA as usize]>()) }
    }
}

#[repr(u8)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum SpecialRegisterName {
    rA = 0, // arithmetic status register
    rB = 1, // TBC
    rC = 2, // TBC
    rD = 3, // TBC
    rE = 4, // TBC
    rF = 5, // TBC
    rG = 6, // TBC
    rH = 7, // TBC
    rI = 8, // TBC
    rJ = 9, // TBC
    rK = 10, // TBC
    rL = 11, // TBC
    rM = 12, // TBC
    rN = 13, // TBC
    rO = 14, // TBC
    rP = 15, // TBC
    rQ = 16, // TBC
    rR = 17, // TBC
    rS = 18, // TBC
    rT = 19, // TBC
    rU = 20, // TBC
    rV = 21, // TBC
    rW = 22, // TBC
    rX = 23, // TBC
    rY = 24, // TBC
    rZ = 25, // TBC
    rBB = 26, // TBC
    rTT = 27, // TBC
    rWW = 28, // TBC
    rXX = 29, // TBC
    rYY = 30, // TBC
    rZZ = 31, // TBC
}

// Machine Instructions
#[allow(dead_code)]
pub struct Instruction {
    op: u8,
    x: u8,
    y: u8,
    z: u8
}

// Useful Constants
#[allow(dead_code)]
pub const MAX_BYTE: u8 = 255;
#[allow(dead_code)]
pub const MAX_WYDE: u16 = 65535;
#[allow(dead_code)]
pub const MAX_TTRA: u32 = 4294967295;
#[allow(dead_code)]
pub const MAX_OCTA: u64 = 18446744073709551615;

