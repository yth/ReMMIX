use std::alloc::{alloc, dealloc, Layout};

// The Machine
pub const GENERAL_PURPOSE_REGISTER_COUNT: usize = 256;
pub const SPECIAL_PURPOSE_REGISTER_COUNT: usize = 32;
pub const MEMORY_SIZE: usize = MAX_TTRA as usize; // 4 GB of main memory

pub type Register = u64;

pub struct MmixMachine {
    pub gp_regs: [Register; GENERAL_PURPOSE_REGISTER_COUNT],
    pub sp_regs: [Register; SPECIAL_PURPOSE_REGISTER_COUNT],
    pub memory: *mut u8 // [0; usize::MAX] is too big for the architecture
}

impl MmixMachine {
    pub fn new() -> Self {
        let layout = Layout::new::<[u8; MEMORY_SIZE]>();

        unsafe {
            Self {
                gp_regs: [0; GENERAL_PURPOSE_REGISTER_COUNT],
                sp_regs: [0; SPECIAL_PURPOSE_REGISTER_COUNT],
                memory: alloc(layout) // [0; usize::MAX] is too big for the architecture
            }
        }
    }

    #[allow(dead_code)]
    pub fn apply<'a>(&'a mut self, instruction: &'a Instruction) -> &mut Self {
        return apply(self, instruction);
    }
}

impl Drop for MmixMachine {
    fn drop(&mut self) {
        unsafe { dealloc(self.memory, Layout::new::<[u8; MEMORY_SIZE]>()) }
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

#[repr(u8)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum OpCode {
    TRAP     = 0x00,
    FCMP     = 0x01,
    FUN      = 0x02,
    FEQL     = 0x03,
    FADD     = 0x04,
    FIX      = 0x05,
    FSUB     = 0x06,
    FIXU     = 0x07,
    FLOT     = 0x08,
    FLOT_I   = 0x09,
    FLOTU    = 0x0A,
    FLOTU_I  = 0x0B,
    SFLOT    = 0x0C,
    SFLOT_I  = 0x0D,
    SFLOTU   = 0x0E,
    SFLOTU_I = 0x0F,
    FMUL     = 0x10,
    FCMPE    = 0x11,
    FUNE     = 0x12,
    FEQLE    = 0x13,
    FDIV     = 0x14,
    FSQRT    = 0x15,
    FREM     = 0x16,
    FINT     = 0x17,
    MUL      = 0x18,
    MUL_I    = 0x19,
    MULU     = 0x1A,
    MULU_I   = 0x1B,
    DIV      = 0x1C,
    DIV_I    = 0x1D,
    DIVU     = 0x1E,
    DIVU_I   = 0x1F,
    ADD      = 0x20,
    ADD_I    = 0x21,
    ADDU     = 0x22,
    ADDU_I   = 0x23,
    SUB      = 0x24,
    SUB_I    = 0x25,
    SUBU     = 0x26,
    SUBU_I   = 0x27,
    ADDU2    = 0x28,
    ADDU2_I  = 0x29,
    ADDU4    = 0x3A,
    ADDU4_I  = 0x3B,
    ADDU8    = 0x3C,
    ADDU8_I  = 0x3D,
    ADDU16   = 0x3E,
    ADDU16_I = 0x3F,
    // More to come
    // ...
    LDB      = 0x80, // Load Instructions
    LDB_I    = 0x81,
    LDBU     = 0x82,
    LDBU_I   = 0x83,
    LDW      = 0x84,
    LDW_I    = 0x85,
    LDWU     = 0x86,
    LDWU_I   = 0x87,
    LDT      = 0x88,
    LDT_I    = 0x89,
    LDTU     = 0x8A,
    LDTU_I   = 0x8B,
    LDO      = 0x8C,
    LDO_I    = 0x8D,
    LDOU     = 0x8E,
    LDOU_I   = 0x8F,

    // More to come
}

// Machine Instructions
pub struct Instruction {
    pub op: OpCode,
    pub x: u8,
    pub y: u8,
    pub z: u8
}

// Useful Constants
#[allow(dead_code)] pub const MAX_NYBB: u8 = 15; // 4 bits, 1/2 byte
#[allow(dead_code)] pub const MAX_BYTE: u8 = 255; // 8 bits, 1 byte
#[allow(dead_code)] pub const MAX_WYDE: u16 = 65_535; // 16 bits, 2 bytes
#[allow(dead_code)] pub const MAX_TTRA: u32 = 4294_967_295; // 32 bits, 4 bytes
#[allow(dead_code)] pub const MAX_OCTA: u64 = 18_446_744_073_709_551_615; // 64 bits, 8 bytes

#[allow(dead_code)] pub const MAX_S_NYBB: i8 = 7;
#[allow(dead_code)] pub const MAX_S_BYTE: i8 = 127;
#[allow(dead_code)] pub const MAX_S_WYDE: i16 = 32_767;
#[allow(dead_code)] pub const MAX_S_TTRA: i32 = 2_147_483_647;
#[allow(dead_code)] pub const MAX_S_OCTA: i64 = 9_223_372_036_854_775_807;

#[allow(dead_code)] pub const MIN_S_NYBB: i8 = -8;
#[allow(dead_code)] pub const MIN_S_BYTE: i8 = -128;
#[allow(dead_code)] pub const MIN_S_WYDE: i16 = -32_768;
#[allow(dead_code)] pub const MIN_S_TTRA: i32 = -2_147_483_648;
#[allow(dead_code)] pub const MIN_S_OCTA: i64 = -9_223_372_036_854_775_808;

#[allow(dead_code)]
pub fn apply<'a>(machine: &'a mut MmixMachine, instruction: &'a Instruction) -> &'a mut MmixMachine {
    match instruction.op {
        OpCode::ADDU_I => return apply_ADDU_I(machine, instruction),
        OpCode::LDB    => return apply_LDB(machine, instruction),
        OpCode::LDB_I  => return apply_LDB_I(machine, instruction),
        OpCode::LDBU   => return apply_LDBU(machine, instruction),
        _ => panic!("Not implemented!")
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn apply_ADDU_I<'a>(m: &'a mut MmixMachine, i: &'a Instruction) -> &'a mut MmixMachine {
    m.gp_regs[i.x as usize] = m.gp_regs[i.y as usize] + i.z as u64;

    return m;
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn apply_LDB<'a>(m: &'a mut MmixMachine, i: &'a Instruction) -> &'a mut MmixMachine {
    let Y = m.gp_regs[i.y as usize];
    let Z = m.gp_regs[i.z as usize];
    let A = Y.wrapping_add(Z) as usize;

    let slice = unsafe { std::slice::from_raw_parts_mut(m.memory, MEMORY_SIZE as usize)};

    m.gp_regs[i.x as usize] = slice[A] as i8 as i64 as u64;

    return m;
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn apply_LDB_I<'a>(m: &'a mut MmixMachine, i: &'a Instruction) -> &'a mut MmixMachine {
    let Y = m.gp_regs[i.y as usize];
    let Z = i.z as u8 as u64;
    let A = Y.wrapping_add(Z) as usize;

    let slice = unsafe { std::slice::from_raw_parts_mut(m.memory, MEMORY_SIZE as usize)};

    m.gp_regs[i.x as usize] = slice[A] as i8 as u64;

    return m;
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn apply_LDBU<'a>(m: &'a mut MmixMachine, i: &'a Instruction) -> &'a mut MmixMachine {
    let Y = m.gp_regs[i.y as usize];
    let Z = m.gp_regs[i.z as usize];
    let A = Y.wrapping_add(Z) as usize;

    let slice = unsafe { std::slice::from_raw_parts_mut(m.memory, MEMORY_SIZE as usize)};

    m.gp_regs[i.x as usize] = slice[A] as u64;

    return m;
}

#[cfg(test)]
pub mod unittests {
    use super::*;

    // Test MmixMachine Initialization
    #[test] #[ignore] // slightly expensive
    fn initialize_all_to_zero() {
        let machine = MmixMachine::new();
        for i in 0..GENERAL_PURPOSE_REGISTER_COUNT {
            assert_eq!(machine.gp_regs[i as usize], 0);
        }

        for i in 0..SPECIAL_PURPOSE_REGISTER_COUNT {
            assert_eq!(machine.sp_regs[i as usize], 0);
        }

        // optimization
        let slice = unsafe { std::slice::from_raw_parts_mut(machine.memory as *mut u128, (MEMORY_SIZE / 16) as usize) };
        for i in 0..(MEMORY_SIZE / 16) {
            assert_eq!(slice[i as usize], 0);
        }
    }

    // Test for working with MmixMachines and Instructions
    #[test]
    fn apply_add_immediate_1() {
        let mut machine = MmixMachine::new();
        let m = &mut machine;
        let instruction = Instruction {
            op: OpCode::ADDU_I,
            x: 0,
            y: 0,
            z: 1,
        };

        assert_eq!(m as *const MmixMachine, apply(m, &instruction) as *const MmixMachine);
        assert_eq!(m.gp_regs[0], 1);
    }

    #[test]
    fn apply_add_immediate_1_chaining() {
        let mut machine = MmixMachine::new();
        let m = &mut machine;
        let instruction = Instruction {
            op: OpCode::ADDU_I,
            x: 0,
            y: 0,
            z: 1,
        };

        assert_eq!(m as *const MmixMachine, apply(apply(m, &instruction), &instruction) as *const MmixMachine);
        assert_eq!(m.gp_regs[0], 2);
    }

    #[test]
    fn apply_add_immediate_1_fold() {
        let mut machine = MmixMachine::new();
        let m = &mut machine;
        let instruction = Instruction {
            op: OpCode::ADDU_I,
            x: 0,
            y: 0,
            z: 1,
        };

        let prog = vec![&instruction, &instruction];

        let m = prog.iter().fold(m, |m, i| apply(m, i));

        assert_eq!(m.gp_regs[0], 2);
    }

    #[test]
    fn mmix_machine_apply_add_immediate_1() {
        let mut machine = MmixMachine::new();
        let instruction = Instruction {
            op: OpCode::ADDU_I,
            x: 0,
            y: 0,
            z: 1,
        };

        machine.apply(&instruction);

        assert_eq!(machine.gp_regs[0], 1);
    }

    #[test]
    fn mmix_machine_apply_add_immediate_1_chaining() {
        let mut machine = MmixMachine::new();
        let instruction = Instruction {
            op: OpCode::ADDU_I,
            x: 0,
            y: 0,
            z: 1,
        };

        machine.apply(&instruction).apply(&instruction);

        assert_eq!(machine.gp_regs[0], 2);
    }

    // Tests for Load Instructions
    #[test]
    fn ldb_simple_test() {
        let mut machine = MmixMachine::new();
        let instruction = Instruction {
            op: OpCode::LDB,
            x: 0,
            y: 0,
            z: 0,
        };

        machine.apply(&instruction);
        assert_eq!(machine.gp_regs[0], 0);
    }

    #[test]
    fn ldb_memory_out_of_range_test() {
        let mut machine = MmixMachine::new();
        machine.gp_regs[1] = MAX_OCTA;
        machine.gp_regs[2] = 1;

        let slice = unsafe { std::slice::from_raw_parts_mut(machine.memory, MEMORY_SIZE as usize)};
        slice[0] = 1;

        let instruction = Instruction {
            op: OpCode::LDB,
            x: 0,
            y: 1,
            z: 2,
        };

        machine.apply(&instruction);
        assert_eq!(machine.gp_regs[0], 1);
    }

    #[test]
    fn ldb_full_range_number_test() {
        let mut machine = MmixMachine::new();
        machine.gp_regs[1] = MAX_OCTA;
        machine.gp_regs[2] = 1;

        let instruction = Instruction {
            op: OpCode::LDB,
            x: 0,
            y: 1,
            z: 2,
        };

        for i in -128..127 {
            let slice = unsafe { std::slice::from_raw_parts_mut(machine.memory, MEMORY_SIZE as usize)};
            slice[0] = i as u8;
            machine.apply(&instruction);
            assert_eq!(machine.gp_regs[0], i as u64);
            assert_eq!(machine.gp_regs[0] as i8, i as i8);
        }
    }

    #[test]
    pub fn ldb_i_full_range_number_test() {
        let mut machine = MmixMachine::new();
        for i in (MIN_S_BYTE as i16)..(MAX_S_BYTE as i16) + 1 {
            let instruction = Instruction {
                op: OpCode::LDB_I,
                x: 0,
                y: 1,
                z: i as u8,
            };

            let slice = unsafe { std::slice::from_raw_parts_mut(machine.memory, MEMORY_SIZE as usize)};
            slice[i as u8 as usize] = i as u8;
            machine.apply(&instruction);
            assert_eq!(machine.gp_regs[0], i as u64);
            assert_eq!(machine.gp_regs[0] as i8, i as i8);
            assert_eq!(machine.gp_regs[0] as i16, i as i16);
        }
    }

    #[test]
    pub fn ldbu_full_range_number_test() {
        let mut machine = MmixMachine::new();
        machine.gp_regs[1] = MAX_OCTA;
        machine.gp_regs[2] = 1;

        let instruction = Instruction {
            op: OpCode::LDBU,
            x: 0,
            y: 1,
            z: 2,
        };

        for i in 0..(MAX_BYTE as u16 + 1) {
            let slice = unsafe { std::slice::from_raw_parts_mut(machine.memory, MEMORY_SIZE as usize)};
            slice[0] = i as u8;
            machine.apply(&instruction);
            assert_eq!(machine.gp_regs[0], i as u64);
            assert_eq!(machine.gp_regs[0] as i8, i as i8);
            assert_eq!(machine.gp_regs[0] as i16, i as i16);
        }
    }
}
