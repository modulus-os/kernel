use disk::Disk;

pub struct Iso9660<T: Disk> {
    disk: T,
    vd_block: u64,
}

impl<T: Disk> Iso9660<T> {
    pub fn new(disk: T, vd_block: u64) -> Option<Self> {
        let buffer = (asm_buffer as *const () as u64) as *mut u16;
        disk.read(vd_block, 1, buffer);
        let identifier: u64 = unsafe { *(buffer as *mut u64).offset(0) };

        // Check for 'CD001' identifier
        if identifier & 0xffffffffff00 == 0x313030444300 {
            return Some(Iso9660 {
                disk: disk,
                vd_block: vd_block,
            });
        } else {
            return None;
        }
    }
}

extern "C" {
    fn asm_buffer();
}
