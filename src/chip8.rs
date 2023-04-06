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
    fn init(&mut self) {
        self.mem.fill(0);
        self.stack.fill(0);
        self.sp = 0;

        self.reg.fill(0);
        self.i_reg = 0;

        self.pc = 0x200;
        self.dt = 0;
        self.st = 0;

        self.display.fill(false);
        self.keys.fill(false);
    }

    fn load_program(&mut self, program: [u8; 3]) {

    }
}
