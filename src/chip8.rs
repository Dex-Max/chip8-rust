pub struct Chip8 {
    mem: [u8; 4096],
    stack: [u16; 16],
    sp: u8,

    reg: [u8; 16],
    i_reg: u16,

    pc: u16,
    op: u16,

    dt: u8,
    st: u8,

    display: [bool; 64 * 32],
    keys: [bool; 16]
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            mem: [0; 4096],
            stack: [0; 16],
            sp: 0,

            reg: [0; 16],
            i_reg: 0,

            pc: 0x200,
            op: 0,

            dt: 0,
            st: 0,

            display: [false; 64 * 32],
            keys: [false; 16]
        }
    }

    fn load_program(&mut self, program: [u8; 3]) {

    }
}
