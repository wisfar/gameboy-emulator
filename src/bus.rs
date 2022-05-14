use crate::mbc::Mbc;

pub struct Bus {
    mbc: Mbc,
}

impl Bus {
    pub fn new(mbc: Mbc) -> Self {
        Bus {
            mbc: mbc
        }
    }
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.mbc.read(addr),
            _ => panic!(),
        }
    }
    pub fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x7FFF => panic!(),
            _ => panic!(),
        }
    }
}