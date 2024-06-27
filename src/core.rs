const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const NUM_KEYS: usize = 16;

// -- CHIP-8 Specification --
pub struct Chip8 {
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

