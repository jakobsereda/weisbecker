const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDRESS: u16 = 0x200;

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

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
    pub fn new() -> Self {
        let mut cpu = Self {
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
        };

        cpu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        cpu
    }

    pub fn reset(&mut self) {
        self.ram = [0; RAM_SIZE];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.dt = 0;
        self.st = 0;
        self.pc = START_ADDRESS;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.display = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        self.ram [..FONTSET_SIZE].copy_from_slice(&FONTSET);
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

