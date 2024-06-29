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

    pub fn tick(&mut self) {
        let op = self.fetch();
        self.execute(op);
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            // SOUND
            self.st -= 1;
        } 
    }

    fn fetch(&mut self) -> u16 {
        let hi = self.ram[self.pc as usize] as u16;
        let lo = self.ram[(self.pc + 1) as usize] as u16;
        self.pc += 2;
        (hi << 8) | lo
    }

    fn execute(&mut self, op: u16) {
        let d1 = (op & 0xF000) >> 12;
        let d2 = (op & 0x0F00) >> 8;
        let d3 = (op & 0x00F0) >> 4;
        let d4 = op & 0x000F;

        match (d1, d2, d3, d4) {
            // -- NOP --
            (0, 0, 0, 0) => return,

            // -- CLS --
            (0, 0, 0xE, 0) => {
                self.display = [false; DISPLAY_WIDTH * DISPLAY_HEIGHT];
            },

            // -- RET --
            (0, 0, 0xE, 0xE) => {
                self.pc = self.pop();
            }, 

            // -- JP addr --
            (1, _, _, _) => {
                self.pc = op & 0xFFF;
            },

            // -- CALL addr --
            (2, _, _, _) => {
                self.push(self.pc);
                self.pc = op & 0xFFF;
            }, 

            // -- SE Vx, byte --
            (3, _, _, _) => {
                let x = d2 as usize;
                let kk = (op & 0xFF) as u8;
                if self.v_reg[x] == kk {
                    self.pc += 2;
                }
            },

            // -- SNE Vx, byte --
            (4, _, _, _) => {
                let x = d2 as usize;
                let kk = (op & 0xFF) as u8;
                if self.v_reg[x] != kk {
                    self.pc += 2;
                }
            },

            // -- SE Vx, Vy -- 
            (5, _, _, 0) => {
                let x = d2 as usize;
                let y = d3 as usize;
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            },

            // -- LD Vx, byte --
            (6, _, _, _) => {
                let x = d2 as usize;
                let kk = (op & 0xFF) as u8;
                self.v_reg[x] = kk;
            },

            // -- ADD Vx, byte --
            (7, _, _, _) => {
                let x = d2 as usize;
                let kk = (op & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(kk);
            },

            // -- LD Vx, Vy --
            (8, _, _, 0) => {
                let x = d2 as usize;
                let y = d3 as usize;
                self.v_reg[x] = self.v_reg[y];
            },

            // -- OR Vx, Vy --
            (8, _, _, 1) => {
                let x = d2 as usize;
                let y = d3 as usize;
                self.v_reg[x] |= self.v_reg[y];
            },

            // -- AND Vx, Vy --
            (8, _, _, 2) => {
                let x = d2 as usize;
                let y = d3 as usize;
                self.v_reg[x] &= self.v_reg[y];
            },

            // -- XOR Vx, Vy --
            (8, _, _, 3) => {
                let x = d2 as usize;
                let y = d3 as usize;
                self.v_reg[x] ^= self.v_reg[y];
            },

            // -- ADD Vx, Vy --
            (8, _, _, 4) => {
                let x = d2 as usize;
                let y = d3 as usize;
                let (sum, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                self.v_reg[x] = sum;
                self.v_reg[0xF] = if carry { 1 } else { 0 };
            },

            // -- SUB Vx, Vy --
            (8, _, _, 5) => {
                let x = d2 as usize;
                let y = d3 as usize;
                let (diff, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                self.v_reg[x] = diff;
                self.v_reg[0xF] = if borrow { 0 } else { 1 };
            },

            // -- SHR Vx {, Vy} --
            (8, _, _, 6) => {
                let x = d2 as usize;
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            },

            // -- SUBN Vx, Vy --
            (8, _, _, 7) => {
                let x = d2 as usize;
                let y = d3 as usize;
                let (diff, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                self.v_reg[x] = diff;
                self.v_reg[0xF] = if borrow { 0 } else { 1 };
            },

            // -- SHL Vx, {, Vy} --
            (8, _, _, 0xE) => {
                let x = d2 as usize;
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            },

            (_, _, _, _) => unimplemented!("Unimplemented opcode: {}", op)
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