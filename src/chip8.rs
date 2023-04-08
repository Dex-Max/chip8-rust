pub enum State {
    Quit,
    Draw,
    Run }

pub struct Chip8 {
    mem: [u8; 4096],
    stack: [u16; 16], sp: u8,
    reg: [u8; 16],
    i_reg: u16,

    pc: u16,
    op: u16,

    dt: u8,
    st: u8,

    pub display: [bool; 64 * 32],
    keys: [bool; 16],

    pub draw_flag: bool
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
            keys: [false; 16],

            draw_flag: false
        }
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.mem[0x200..(0x200 + program.len())].copy_from_slice(program.as_slice());
    }

    pub fn cycle(&mut self) -> State {
        if self.pc >= 4096 {
            return State::Quit;
        }

        println!("OP: {:x}", self.op);
        println!("PC: {:x}", self.pc);
        println!("Regs:");
        for i in 0..16 {
            println!("V{}: {}", i, self.reg[i]);
        }

        self.op = ((self.mem[self.pc as usize] as u16) << 8) | (self.mem[(self.pc + 1) as usize]) as u16;
        self.pc += 2;
        self.execute_instruction();

        if self.draw_flag {
            return State::Draw;
        } else {
            return State::Run;
        }
    }

    fn execute_instruction(&mut self) {
        match self.op >> 12 {
            0 => {
                match self.op & 0xff {
                    0xE0 => {
                        self.display = [false; 64 * 32];
                    }
                    _ => {

                    }
                }
            }
            1 => {
                self.pc = self.op & 0xfff;
            }
            6 => {
                self.reg[((self.op >> 8) & 0xf) as usize] = (self.op & 0xff) as u8;
            }
            7 => {
                self.reg[((self.op >> 8) & 0xf) as usize] += (self.op & 0xff) as u8;
            }
            0xA => {
                self.i_reg = self.op & 0xfff;
            }
            0xD => {
                let x = self.reg[((self.op >> 8) & 0xf) as usize] % 64;
                let y = self.reg[((self.op >> 4) & 0xf) as usize] % 32;
                let n = self.op & 0xf;

                self.reg[0xf] = 0;

                for i in 0..n {
                    let sprite = self.mem[(self.i_reg + i) as usize];

                    for p in 0..8 {
                        let pixel_index = ((y as u16 + i) * 64 + (x as u16 + p)) as usize;

                        if pixel_index < 64 * 32 && x as u16 + p < 64 {
                            let on = if (sprite >> (7 - p)) & 1 == 0 { false } else { true }; 
                            if self.display[pixel_index] && on {
                                self.reg[0xf] = 1;
                            }

                            self.display[pixel_index] ^= on; 
                        }
                    }
                }

                self.draw_flag = true;
            }
            _ => {
            }

        }
    }
}
