#![no_std]

use bitmatch::bitmatch;

struct Cpu {
    a: u8,
    f: Flag,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,

    mem: [u8; 0xffff],
}

struct Flag {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            f: Flag { z: false, n: false, h: false, c: false },
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0xfffe,
            pc: 0x0100,
            mem: [0; 0xffff],
        }
    }

    fn tick(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    fn write_a(&mut self, x: u8) {
        self.a = x;
    }
    fn write_b(&mut self, x: u8) {
        self.bc &= 0x00FF;
        self.bc |= (x as u16) << 8;
    }
    fn write_c(&mut self, x: u8) {
        self.bc &= 0xFF00;
        self.bc |= x as u16;
    }
    fn write_d(&mut self, x: u8) {
        self.de &= 0x00FF;
        self.de |= (x as u16) << 8;
    }
    fn write_e(&mut self, x: u8) {
        self.de &= 0xFF00;
        self.de |= x as u16;
    }
    fn write_h(&mut self, x: u8) {
        self.hl &= 0x00FF;
        self.hl |= (x as u16) << 8;
    }
    fn write_l(&mut self, x: u8) {
        self.hl &= 0xFF00;
        self.hl |= x as u16;
    }
    fn read_a(&self) -> u8 {
        self.a
    }
    fn read_b(&self) -> u8 {
        (self.bc >> 8) as u8
    }
    fn read_c(&self) -> u8 {
        (self.bc & 0x00FF) as u8
    }
    fn read_d(&self) -> u8 {
        (self.de >> 8) as u8
    }
    fn read_e(&self) -> u8 {
        (self.bc & 0x00FF) as u8
    }
    fn read_h(&self) -> u8 {
        (self.hl >> 8) as u8
    }
    fn read_l(&self) -> u8 {
        (self.hl & 0x00FF) as u8
    }

    #[bitmatch]
    fn execute(&mut self, opcode: u8) {
        #[bitmatch]
        match &opcode {
            // r : 8-bit  register
            // rr: 16-bit register
            // n : 8-bit  immediate value
            // nn: 16-bit immediate value
            "00000000" => self.nop(),
            //"00010000" => self.stop(),
            //"01110110" => self.halt(),

            // 8-bit load
            "01xxxyyy" => self.ld_r(x, y),
            "00xxx110" => self.ld_r_n(x),
            //"00110110" => self.ld_hl_n(x),

            // 16-bit load
            //"00xx0001" => self.ld_rr_nn(x),
            //"00001000" => self.ld_nn_sp(x),
            //"11xx0101" => self.push_rr(x),
            //"11xx0001" => self.pop_rr(x),
            // 8-bit arithmetc/logical instructions
            // 16-bit arithmetc/logical instructions
            // jump, call
            // return
            // reset
            _ => panic!(),
        }
    }

    fn nop(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }
    //fn stop() {}
    //fn halt() {}
    fn ld_r(&mut self, x: u8, y: u8) {
        let src = match y {
            0b000 => self.read_b(),
            0b001 => self.read_c(),
            0b010 => self.read_d(),
            0b011 => self.read_e(),
            0b100 => self.read_h(),
            0b101 => self.read_l(),
            0b110 => self.mem[self.hl as usize],
            0b111 => self.read_a(),
            _ => panic!(),
        };
        match x {
            0b000 => self.write_b(src),
            0b001 => self.write_c(src),
            0b010 => self.write_d(src),
            0b011 => self.write_e(src),
            0b100 => self.write_h(src),
            0b101 => self.write_l(src),
            0b110 => self.mem[self.hl as usize] = src,
            0b111 => self.write_a(src),
            _ => panic!(),
        }
        self.pc = self.pc.wrapping_add(1);
    }
    fn ld_r_n(&mut self, x: u8) {
        let data = self.mem[self.pc as usize + 1];
        match x {
            0b000 => self.write_b(data),
            0b001 => self.write_c(data),
            0b010 => self.write_d(data),
            0b011 => self.write_e(data),
            0b100 => self.write_h(data),
            0b101 => self.write_l(data),
            0b110 => self.mem[self.hl as usize] = data,
            0b111 => self.write_a(data),
            _ => panic!(),
        }
    }
    fn ld_r_m() {}
    fn ld_m_r() {}
    fn push_rr() {}
    fn pop_rr() {}
}

#[test]
fn test_cpu() {
    let mut cpu = Cpu::new();
    assert_eq!(cpu.a, 0);
    cpu.bc = 0x0100;
    cpu.execute(0x78);
    assert_eq!(cpu.a, 1);
    cpu.execute(0x3e);
    assert_eq!(cpu.a, 0);
}