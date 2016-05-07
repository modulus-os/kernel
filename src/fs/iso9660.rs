use disk::Disk;

pub struct Iso9660<T: Disk> {
    disk: T,
    pvd: [u8; 2048],
}

pub struct PathEntry {
    len: u8,
    earl: u8,
    location: u32,
    num_parents: u16,
}

impl<T: Disk> Iso9660<T> {
    pub fn new(disk: T) -> Option<Self> {
        let mut buffer = [0u8; 2048];
        disk.read(0x40, 1, &mut buffer);

        let id = unsafe { *(&buffer as *const u8 as *const u64) } & 0xffffffffff00;

        // Check for 'CD001' identifier
        if id == 0x313030444300 {
            return Some(Iso9660 {
                disk: disk,
                pvd: buffer,
            });
        } else {
            return None;
        }
    }

    pub fn find(&self) {
        let lpt = 132;
        let location = (self.pvd[lpt + 3] as u32) << 24 | (self.pvd[lpt + 2] as u32) << 16 |
                       (self.pvd[lpt + 1] as u32) << 8 |
                       (self.pvd[lpt] as u32);
        print!("{}", location);

        // let mut buffer = [0u8; 512];
        // self.disk.read(location as u64, 1, &mut buffer);

        // print!("Buffer: {}", buffer[0]);
    }
}
