#![no_std]

pub struct Mbc {
    header: Header,
    rom: [u8; 0x8000],
}

#[derive(Debug)]
struct Header {
    entry_point: [u8; 4],
    nintendo_logo: [u8; 48],
    title: [u8; 16],
    manufacturer_code: [u8; 4],
    cgb_flag: u8,
    new_licensee_code: [u8; 2],
    sgb_flag: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    old_licnsee_code: u8,
    mask_rom_version_number: u8,
    header_checksum: u8,
    global_checksum: [u8; 2],
}

impl Mbc {
    pub fn new(data: &[u8]) -> Self {
        let header = Header {
            entry_point: data[0x0100..=0x0103].try_into().unwrap(),
            nintendo_logo: data[0x0104..=0x0133].try_into().unwrap(),
            title: data[0x0134..=0x0143].try_into().unwrap(),
            manufacturer_code: data[0x013f..=0x0142].try_into().unwrap(),
            cgb_flag: data[0x0143],
            new_licensee_code: data[0x0144..=0x0145].try_into().unwrap(),
            sgb_flag: data[0x0146],
            cartridge_type: data[0x0147],
            rom_size: data[0x0148],
            ram_size: data[0x0149],
            destination_code: data[0x014a],
            old_licnsee_code: data[0x014b],
            mask_rom_version_number: data[0x014c],
            header_checksum: data[0x014d],
            global_checksum: data[0x014e..=0x014f].try_into().unwrap(),
        };
        Mbc {
            header: header,
            rom: data[0x0000..=0x7FFF].try_into().unwrap(),
        }
    }
    pub fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
}

#[test]
fn test_rom() {
    use std::fs::File;
    use std::io::Read;

    let mut path = String::from(env!("CARGO_MANIFEST_DIR"));
    path.push_str("/gb-test-roms/cpu_instrs/cpu_instrs.gb");
    let mut f = File::open(path).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf);
    let mbc = Mbc::new(&buf);
    println!("{:x?}", mbc.header);

    let mut path = String::from(env!("CARGO_MANIFEST_DIR"));
    path.push_str("/gb-hello-world/hello-world.gb");
    let mut f = File::open(path).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf);
    let mbc = Mbc::new(&buf);
    println!("{:x?}", mbc.header);
}