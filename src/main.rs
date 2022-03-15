use gameboy_emulator::memory::Memory;

fn main() {
    let mut memory = Memory::default();
    assert_eq!(0, memory.read(0));
    memory.write(1, 0);
    assert_eq!(1, memory.read(0));
}
