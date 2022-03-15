pub struct Memory {
    mem: [u8; 0x10000],
}

impl Default for Memory {
    fn default() -> Self {
        Memory::new()
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: [0; 0x10000],
        }
    }

    pub fn write(&mut self, data: u8, addr: usize) {
        if addr < 0x10000 {
            self.mem[addr] = data;
        } else {
            panic!("Write: Out of memory!");
        }
    }

    pub fn read(&self, addr: usize) -> u8 {
        if addr < 0x10000 {
            self.mem[addr]
        } else {
            panic!("Read: Out of memory!");
        }
    }
}

#[test]
fn test_memory() {
    let mut memory = Memory::default();
    assert_eq!(0, memory.read(0));
    memory.write(1, 0);
    assert_eq!(1, memory.read(0));
}