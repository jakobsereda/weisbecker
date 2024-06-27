const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDRESS: u16 = 0x200;

// -- CHIP-8 CPU Specification --
pub struct CPU {
    // -- memory --
    ram: [u8; RAM_SIZE],

    // -- general purpose registers --
    v_reg: [u8; NUM_REGS],

    // -- index register --
    i_reg: u16,

    // -- delay timer --
    dt: u8,

    // -- sound timer --
    st: u8,

    // -- program counter --
    pc: u16,

    // -- stack pointer --
    sp: u8,

    // -- stack --
    stack: [u16; STACK_SIZE],

    // -- keyboard --
    keys: [bool; NUM_KEYS],

    // -- display (64px x 32px) --
    display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT]
}

impl CPU {
    // -- constructor --
    pub fn new() -> Self {
        Self {
            ram: [0; RAM_SIZE],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            dt: 0,
            st: 0,
            pc: START_ADDRESS,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT]
        }
    }


    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }
}

