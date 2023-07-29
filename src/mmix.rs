use std::alloc::{alloc, dealloc, Layout};
#[allow(unused_imports)] use std::ptr;

// The Machine
pub const GENERAL_PURPOSE_REGISTER_COUNT: u32 = 256;
pub const MEMORY_SIZE: u32 = MAX_TTRA as u32; // 4 GB of main memory

pub type Register = u64;

pub struct MmixMachine {
    pub gp_regs: [Register; GENERAL_PURPOSE_REGISTER_COUNT as usize],
    pub sp_regs: [Register; 32],
    pub memory: *mut u8 // [0; usize::MAX] is too big for the architecture
}

impl MmixMachine {
    pub fn new() -> Self {
        let layout = Layout::new::<[u8; MAX_TTRA as usize]>();

        unsafe {
            Self {
                gp_regs: [0; GENERAL_PURPOSE_REGISTER_COUNT as usize],
                sp_regs: [0; 32],
                memory: alloc(layout) // [0; usize::MAX] is too big for the architecture
            }
        }
    }

    pub fn apply<'a>(&'a mut self, instruction: &'a Instruction) -> &mut Self {
        return apply(self, instruction);
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
}

// Machine Instructions
#[allow(dead_code)]
pub struct Instruction {
    op: OpCode,
    x: u8,
    y: u8,
    z: u8
}

// Useful Constants
#[allow(dead_code)] pub const MAX_BYTE: u8 = 255;
#[allow(dead_code)] pub const MAX_WYDE: u16 = 65535;
#[allow(dead_code)] pub const MAX_TTRA: u32 = 4294967295;
#[allow(dead_code)] pub const MAX_OCTA: u64 = 18446744073709551615;

#[allow(dead_code)]
pub fn apply<'a>(machine: &'a mut MmixMachine, instruction: &'a Instruction) -> &'a mut MmixMachine {
    match instruction.op {
        OpCode::ADDU_I => return apply_ADDU_I(machine, instruction),
        _ => panic!("Not implemented!"),
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
fn apply_ADDU_I<'a>(m: &'a mut MmixMachine, i: &'a Instruction) -> &'a mut MmixMachine {
    m.gp_regs[i.x as usize] = m.gp_regs[i.y as usize] + i.z as u64;

    return m;
}

#[cfg(test)]

mod unittests {
    use super::*;

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
}
