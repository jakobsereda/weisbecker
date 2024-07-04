use rand::random;

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
    ram: [u8; RAM_SIZE],
    v_reg: [u8; NUM_REGS],
    i_reg: u16,
    dt: u8,
    st: u8,
    pc: u16,
    sp: u8,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
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

    pub fn get_display(&self) -> &[bool] {
        &self.display
    }

    pub fn key_press(&mut self, n: usize, state: bool) {
        self.keys[n] = state;
    }

    pub fn load(&mut self, data: &[u8]) {
        let start = START_ADDRESS as usize;
        let end = (START_ADDRESS as usize) + data.len();
        self.ram[start..end].copy_from_slice(data);
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

            // -- SNE Vx, Vy --
            (9, _, _, 0) => {
                let x = d2 as usize;
                let y = d3 as usize;
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2;
                }
            },

            // -- LD I, addr --
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
            },

            // -- JP V0, addr --
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn + (self.v_reg[0] as u16);
            },

            // -- RND Vx, byte --
            (0xC, _, _, _) => {
                let x = d2 as usize;
                let kk = (op & 0xFF) as u8;
                let rand: u8 = random();
                self.v_reg[x] = rand & kk;
            },

            // -- DRW Vx, Vy, nibble --
            (0xD, _, _, _) => {
                let x_coord = self.v_reg[d2 as usize] as u16;
                let y_coord = self.v_reg[d3 as usize] as u16;
                let mut flipped = false;
                for row in 0..=d4 {
                    let addr = self.i_reg + row;
                    let data = self.ram[addr as usize];
                    for column in 0..8 {
                        if (data & (0b1000_000 >> column)) != 0 {
                            let x = (x_coord + column) as usize % DISPLAY_WIDTH;
                            let y = (y_coord + row) as usize % DISPLAY_HEIGHT;
                            let n = x + DISPLAY_WIDTH * y;
                            flipped |= self.display[n];
                            self.display[n] ^= true;
                        }
                    }
                }
                self.v_reg[0xF] = if flipped { 1 } else { 0 };
            },

            // -- SKP Vx --
            (0xE, _, 9, 0xE) => {
                let x = d2 as usize;
                let val = self.v_reg[x] as usize;
                if self.keys[val] {
                    self.pc += 2;
                }
            },

            // -- SKNP Vx --
            (0xE, _, 0xA, 1) => {
                let x = d2 as usize;
                let val = self.v_reg[x] as usize;
                if !self.keys[val] {
                    self.pc += 2;
                }
            },

            // -- LD Vx, DT --
            (0xF, _, 0, 7) => {
                let x = d2 as usize;
                self.v_reg[x] = self.dt;
            },

            // -- LD Vx, K --
            (0xF, _, 0, 0xA) => {
                let x = d2 as usize;
                let mut pressed = false;
                for i in 0..self.keys.len() {
                    if self.keys[i] {
                        self.v_reg[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }
                if !pressed {
                    self.pc -= 2;
                }
            },

            // -- LD DT, Vx --
            (0xF, _, 1, 5) => {
                let x = d2 as usize;
                self.dt = self.v_reg[x];
            },

            // -- LD ST, Vx --
            (0xF, _, 1, 8) => {
                let x = d2 as usize;
                self.st = self.v_reg[x];
            },

            // -- ADD I, Vx --
            (0xF, _, 1, 0xE) => {
                let x = d2 as usize;
                self.i_reg = self.i_reg.wrapping_add(self.v_reg[x] as u16);
            },

            // -- LD F, Vx -- 
            (0xF, _, 2, 9) => {
                let x = d2 as usize;
                self.i_reg = (self.v_reg[x] as u16) * 5;
            },

            // -- LD B, Vx --
            (0xF, _, 3, 3) => {
                let x = d2 as usize;
                let vx = self.v_reg[x] as f32;
                self.ram[self.i_reg as usize] = (vx / 100.0).floor() as u8;
                self.ram[(self.i_reg + 1) as usize] = ((vx / 10.0) % 10.0).floor() as u8;
                self.ram[(self.i_reg + 2) as usize] = (vx % 10.0) as u8;
            },

            // -- LD [I], Vx --
            (0xF, _, 5, 5) => {
                let x = d2 as usize;
                for n in 0..=x {
                    self.ram[(self.i_reg as usize) + n] = self.v_reg[n];
                }
            },

            // -- LD Vx, [I] --
            (0xF, _, 6, 5) => {
                let x = d2 as usize;
                for n in 0..=x {
                    self.v_reg[n] = self.ram[(self.i_reg as usize) + n];
                }
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