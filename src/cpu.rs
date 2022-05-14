#![no_std]

use crate::bus::Bus;

#[derive(Clone, Copy)]
enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy)]
enum Reg16 {
    Bc,
    De,
    Hl,
    Sp,
}

struct Cpu {
    a: u8,
    f: Flag,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16,
    bus: Bus,
}

struct Flag {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl Cpu {
    pub fn new(bus: Bus) -> Self {
        Cpu {
            a: 0,
            f: Flag {
                z: false,
                n: false,
                h: false,
                c: false,
            },
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0xfffe,
            pc: 0x0100,
            bus: bus,
        }
    }

    fn write_r8(&mut self, reg: Reg8, data: u8) {
        match reg {
            Reg8::A => {
                self.a = data;
            }
            Reg8::B => {
                self.bc &= 0x00FF;
                self.bc |= (data as u16) << 8;
            }
            Reg8::C => {
                self.bc &= 0xFF00;
                self.bc |= data as u16;
            }
            Reg8::D => {
                self.de &= 0x00FF;
                self.de |= (data as u16) << 8;
            }
            Reg8::E => {
                self.de &= 0xFF00;
                self.de |= data as u16;
            }
            Reg8::H => {
                self.hl &= 0x00FF;
                self.hl |= (data as u16) << 8;
            }
            Reg8::L => {
                self.hl &= 0xFF00;
                self.hl |= data as u16;
            }
            _ => panic!(),
        }
    }
    fn write_r16(&mut self, reg: Reg16, data: u16) {
        match reg {
            Reg16::Bc => {
                self.bc = data;
            }
            Reg16::De => {
                self.de = data;
            }
            Reg16::Hl => {
                self.hl = data;
            }
            Reg16::Sp => {
                self.sp = data;
            }
            _ => panic!(),
        }
    }
    fn read_r8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::A => self.a,
            Reg8::B => (self.bc >> 8) as u8,
            Reg8::C => self.bc as u8,
            Reg8::D => (self.de >> 8) as u8,
            Reg8::E => self.de as u8,
            Reg8::H => (self.hl >> 8) as u8,
            Reg8::L => self.hl as u8,
        }
    }
    fn read_r16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::Bc => self.bc,
            Reg16::De => self.de,
            Reg16::Hl => self.hl,
            Reg16::Sp => self.sp,
        }
    }

    fn execute(&mut self, opcode: u8) {
        match &opcode {
            0x00 => self.nop(),
            0x01 => self.ld_d16_into_r16(Reg16::Bc),
            0x02 => self.ld_r8_into_r16a(Reg8::A, Reg16::Bc),
            0x03 => self.inc_r16(Reg16::Bc),
            0x04 => self.inc_r8(Reg8::B),
            0x05 => self.dec_r8(Reg8::B),
            0x06 => self.ld_d8_into_r8(Reg8::B),
            //0x07 => self.rlc_r8(Reg8::A),
            //0x08 => self.ld_a16_into_sp(),
            //0x09 => self.add_r16_to_hl(Reg16::Bc),
            //0x0a => self.ld_r16_addr_into_r8(Reg16::Bc, Reg8::A),
            0x0b => self.dec_r16(Reg16::Bc),
            0x0c => self.inc_r8(Reg8::C),
            //0x0d => self.dec_r8(Reg8::C),
            //0x0e => self.ld_d8_into_r8(Reg8::C),
            //0x0f => self.rrc_r8(Reg8::A),
            //0x10 => self.stop(),
            //0x11 => self.ld_d16_into_r16(Reg16::De),
            //0x12
            0x13 => self.inc_r16(Reg16::De),
            0x14 => self.inc_r8(Reg8::D),
            0x1c => self.inc_r8(Reg8::E),
            0x23 => self.inc_r16(Reg16::Hl),
            0x24 => self.inc_r8(Reg8::H),
            0x2c => self.inc_r8(Reg8::L),
            0x33 => self.inc_r16(Reg16::Sp),
            0x34 => self.inc_r16a(Reg16::Hl),
            0x3c => self.inc_r8(Reg8::A),
            0x3e => self.ld_d8_into_r8(Reg8::A),
            //0x76 => self.halt(),
            0x78 => self.ld_r8_into_r8(Reg8::B, Reg8::A),
            _ => panic!(),
        }
    }
    fn write_m8(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data);
    }
    fn read_m8(&self, addr: u16) -> u8 {
        self.bus.read(addr)
    }
    fn read_m16(&self, addr: u16) -> u16 {
        let lsb = self.read_m8(addr);
        let msb = self.read_m8(addr + 1);
        ((msb as u16) << 8) & (lsb as u16)
    }
    fn nop(&mut self) {
        self.pc += 1;
    }
    // 8-bit load
    fn ld_r8_into_r8(&mut self, src: Reg8, dest: Reg8) {
        let data = self.read_r8(src);
        self.write_r8(dest, data);
        self.pc += 1;
    }
    fn ld_d8_into_r8(&mut self, dest: Reg8) {
        self.pc += 1;
        let data = self.read_m8(self.pc);
        self.write_r8(dest, data);
        self.pc += 1;
    }
    fn ld_r8_into_r16a(&mut self, src: Reg8, dest: Reg16) {
        let addr = self.read_r16(dest);
        let data = self.read_r8(src);
        self.write_m8(addr, data);
    }
    fn ldh_r8_into_a8() {}
    fn ldh_a8_into_r8() {}
    // 16-bit load
    fn ld_d16_into_r16(&mut self, dest: Reg16) {
        self.pc += 1;
        let data = self.read_m16(self.pc);
        self.write_r16(dest, data);
        self.pc += 2;
    }
    fn ld_sp_into_a16() {}
    fn stop() {}
    fn halt() {}
    fn push() {}
    fn pop() {}
    fn is_half_carry_add(left: u8, right: u8) -> bool {
        (left & 0x0F) + (right & 0x0F) > 0x0F
    }
    fn is_half_carry_sub(left: u8, right: u8) -> bool {
        (left & 0x0F) < (right & 0x0F)
    }
    fn inc_r8(&mut self, reg: Reg8) {
        let left = self.read_r8(reg);
        let res = left.wrapping_add(1);
        self.write_r8(reg, res);
        self.set_flags(
            Some(res == 0),
            Some(false),
            Some(Self::is_half_carry_add(left, 1)),
            None,
        )
    }
    fn inc_r16(&mut self, reg: Reg16) {
        let left = self.read_r16(reg);
        let res = left.wrapping_add(1);
        self.write_r16(reg, res);
        // 16-bit inc do not affect flags
    }
    fn inc_r16a(&mut self, reg: Reg16) {
        let addr = self.read_r16(reg);
        let left = self.read_m8(addr);
        let res = left.wrapping_add(1);
        self.write_m8(addr, res);
        self.set_flags(
            Some(res == 0),
            Some(false),
            Some(Self::is_half_carry_add(left, 1)),
            None,
        )
    }
    fn dec_r8(&mut self, reg: Reg8) {
        let left = self.read_r8(reg);
        let res = left.wrapping_sub(1);
        self.write_r8(reg, res);
        self.set_flags(
            Some(res == 0),
            Some(true),
            Some(Self::is_half_carry_sub(left, 1)),
            None,
        )
    }
    fn dec_r16(&mut self, reg: Reg16) {
        let left = self.read_r16(reg);
        let res = left.wrapping_sub(1);
        self.write_r16(reg, res);
        self.set_flags(
            Some(res == 0),
            Some(true),
            Some(Self::is_half_carry_sub(left, 1)),
            None,
        )
    }

    fn set_flags(&mut self, z: Option<bool>, n: Option<bool>, h: Option<bool>, c: Option<bool>) {
        if let Some(z) = z {
            self.f.z = z
        }
        if let Some(n) = n {
            self.f.n = n
        }
        if let Some(h) = h {
            self.f.h = h
        }
        if let Some(c) = c {
            self.f.c = c
        }
    }
}

#[test]
fn test_cpu() {
    use crate::mbc::Mbc;
    use std::fs::File;
    use std::io::Read;

    let mut path = String::from(env!("CARGO_MANIFEST_DIR"));
    path.push_str("/gb-hello-world/hello-world.gb");
    let mut f = File::open(path).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf);
    let mut mbc = Mbc::new(&buf);
    let mut cpu = Cpu::new(Bus::new(mbc));
    assert_eq!(cpu.a, 0);

    // ld
    cpu.bc = 0x0100;
    cpu.execute(0x78);
    assert_eq!(cpu.a, 1);
    cpu.execute(0x3e);
    assert_eq!(cpu.a, 0);

    // inc
    let prev = cpu.bc;
    cpu.execute(0x03); // inc bc
    assert_eq!(cpu.bc, prev + 0x0001);
    let prev = cpu.bc;
    cpu.execute(0x04); // inc b
    assert_eq!(cpu.bc, prev + 0x0100);
    let prev = cpu.bc;
    cpu.execute(0x0c); //inc c
    assert_eq!(cpu.bc, prev + 0x0001);
    let prev = cpu.de; // inc de
    cpu.execute(0x13);
    assert_eq!(cpu.de, prev + 0x0001);
}
