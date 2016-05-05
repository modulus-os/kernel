use disk::Disk;

pub struct Iso9660<T: Disk> {
    disk: T,
    pvd: [u8; 2048],
}

impl<T: Disk> Iso9660<T> {
    pub fn new(disk: T, vd_block: u64) -> Option<Self> {
        let mut buffer: [u8; 2048] = [0; 2048];
        disk.read(vd_block, 1, &mut buffer);

        let id = unsafe { *(&buffer as *const u8 as *const u64) };

        // Check for 'CD001' identifier
        if id & 0xffffffffff00 == 0x313030444300 {
            return Some(Iso9660 {
                disk: disk,
                pvd: buffer,
            });
        } else {
            return None;
        }
    }

    pub fn read_root(&self) {
        print!("{:0x}\n", self.pvd[1]);
    }
}
