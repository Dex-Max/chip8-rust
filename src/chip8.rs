use rand::Rng;

pub enum State {
    Quit,
    Draw,
    Run 
}

pub struct Chip8 {
    mem: [u8; 4096],
    stack: [u16; 16], 
    sp: u8,

    reg: [u8; 16],
    i_reg: u16,

    pc: u16,
    op: u16,

    pub dt: u8,
    pub st: u8,

    pub display: [bool; 64 * 32],
    keys: [bool; 16],

    pub draw_flag: bool
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let font: [u8; 80] = [
            0xf0, 0x90, 0x90, 0x90, 0xf0,
            0x20, 0x60, 0x20, 0x20, 0x70,
            0xf0, 0x10, 0xf0, 0x80, 0xf0,
            0xf0, 0x10, 0xf0, 0x10, 0xf0,
            0x90, 0x90, 0xf0, 0x10, 0x10,
            0xf0, 0x80, 0xf0, 0x10, 0xf0,
            0xf0, 0x80, 0xf0, 0x90, 0xf0,
            0xf0, 0x10, 0x20, 0x40, 0x40,
            0xf0, 0x90, 0xf0, 0x90, 0xf0,
            0xf0, 0x90, 0xf0, 0x10, 0xf0,
            0xf0, 0x90, 0xf0, 0x90, 0x90,
            0xe0, 0x90, 0xe0, 0x90, 0xe0,
            0xf0, 0x80, 0x80, 0x80, 0xf0,
            0xe0, 0x90, 0x90, 0x90, 0xe0,
            0xf0, 0x80, 0xf0, 0x80, 0xf0,
            0xf0, 0x80, 0xf0, 0x80, 0x80
        ];

        let mut mem = [0; 4096];
        &mem[0..80].copy_from_slice(font.as_slice());

        Chip8 {
            mem,
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

    fn push_stack(&mut self, val: u16) {
        self.stack[self.sp as usize] = val; 
        self.sp += 1;
    }

    fn pop_stack(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.mem[0x200..(0x200 + program.len())].copy_from_slice(program.as_slice());
    }

    pub fn cycle(&mut self) -> State {
        //let mut line = String::new();
        //let _ = std::io::stdin().read_line(&mut line).unwrap();
        if self.pc >= 4096 {
            return State::Quit;
        }

        println!("*****************");
        println!("OP: {:x}", self.op);
        println!("PC: {:x}", self.pc);
        println!("Regs:");
        for i in 0..16 {
            println!("V{}: {:x}", i, self.reg[i]);
        }
        println!("SP: {}", self.sp);

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
        let nnn = self.op & 0xfff;
        let n = self.op & 0xf;
        let x = ((self.op >> 8) & 0xf) as usize;
        let y = ((self.op >> 4) & 0xf) as usize;
        let kk = (self.op & 0xff) as u8;

        match self.op >> 12 {
            0 => {
                match self.op & 0xff {
                    0xE0 => {
                        self.display = [false; 64 * 32];
                    }
                    0xEE => {
                        self.pc = self.pop_stack();
                    }
                    _ => {
                        panic!("Invalid opcode");
                    }
                }
            }
            1 => {
                self.pc = nnn;
            }
            2 => {
                self.push_stack(self.pc);
                self.pc = nnn;
            }
            3 => {
                if self.reg[x] == kk {
                    self.pc += 2;
                }
            }
            4 => {
                if self.reg[x] != kk {
                    self.pc += 2;
                }
            }
            5 => {
                if self.reg[x] == self.reg[y] {
                    self.pc += 2;
                }
            }
            6 => {
                self.reg[x] = kk;
            }
            7 => {
                self.reg[x] = self.reg[x].wrapping_add(kk);
            }
            8 => {
                match n {
                    0 => {
                        self.reg[x] = self.reg[y];
                    }
                    1 => {
                        self.reg[x] |= self.reg[y];
                    }
                    2 => {
                        self.reg[x] &= self.reg[y];
                    }
                    3 => {
                        self.reg[x] ^= self.reg[y];
                    }
                    4 => {
                        let temp = self.reg[x];
                        self.reg[x] = self.reg[x].wrapping_add(self.reg[y]);

                        self.reg[0xf] = if self.reg[x] < temp { 1 } else { 0 };
                    }
                    5 => {
                        self.reg[0xf] = if self.reg[x] > self.reg[y] { 1 } else { 0 };
                        self.reg[x] = self.reg[x].wrapping_sub(self.reg[y]);
                    }
                    6 => {
                        self.reg[0xf] = self.reg[x] & 1;
                        self.reg[x] >>= 1;
                    }
                    7 => {
                        self.reg[0xf] = if self.reg[x] < self.reg[y] { 1 } else { 0 };
                        self.reg[x] = self.reg[y].wrapping_sub(self.reg[x]);
                    }
                    0xE => {
                        self.reg[0xf] = self.reg[x] >> 7;
                        self.reg[x] <<= 1;
                    }
                    _ => {
                        panic!("Invalid opcode");
                    }
                }
            }
            9 => {
                if self.reg[x] != self.reg[y] {
                    self.pc += 2;
                }
            }
            0xA => {
                self.i_reg = nnn;
            }
            0xB => {
                self.pc = nnn + self.reg[0] as u16;
            }
            0xC => {
                let random = rand::thread_rng().gen_range(0..=255);
                self.reg[x] = random & kk;
            }
            0xD => {
                let xc = (self.reg[x] % 64) as u16;
                let yc = (self.reg[y] % 32) as u16;
                self.reg[0xf] = 0;

                for i in 0..n {
                    let sprite = self.mem[(self.i_reg + i) as usize];

                    for p in 0..8 {
                        let pixel_index = ((yc + i) * 64 + (xc + p)) as usize;

                        if pixel_index < 64 * 32 && xc + p < 64 {
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
            0xE => {
                panic!("Not implemented");
            }
            0xF => {
                match kk {
                    7 => {
                        self.reg[x] = self.dt;
                    }
                    0xA => {
                        panic!("Not implemented");
                    }
                    0x15 => {
                        self.dt = self.reg[x];
                    }
                    0x18 => {
                        self.st = self.reg[x];
                    }
                    0x1E => {
                        self.i_reg = self.i_reg.wrapping_add(self.reg[x] as u16);
                    }
                    0x29 => {
                        self.i_reg = self.reg[x] as u16 * 5;
                    }

                    0x33 => {
                        self.mem[self.i_reg as usize] = self.reg[x] / 100;
                        self.mem[(self.i_reg + 1) as usize] = (self.reg[x] % 100) / 10;
                        self.mem[(self.i_reg + 2) as usize] = self.reg[x] % 10;
                    }
                    0x55 => {
                        for i in 0..(x + 1) {
                            self.mem[(self.i_reg + i as u16) as usize] = self.reg[i];
                        }
                    }
                    0x65 => {
                        for i in 0..(x + 1) {
                            self.reg[i] = self.mem[(self.i_reg + i as u16) as usize];
                        }
                    }
                    _ => {
                        panic!("Invalid opcode");
                    }
                }
            }
            _ => {
                panic!("Invalid opcode");
            }
        }
    }
}
