use disk::Disk;

pub struct Iso9660<T: Disk> {
    disk: T,
    pvd: [u8; 2048],
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
        let byte = 158;
        let location = (self.pvd[byte + 3] as u32) << 24 | (self.pvd[byte + 2] as u32) << 16 |
                       (self.pvd[byte + 1] as u32) << 8 |
                       (self.pvd[byte] as u32);
        print!("{:0x}\n", location * 4);

        let mut buffer = [0u8; 512];
        self.disk.read((location * 4) as u64, 1, &mut buffer);

        let mut directory_iter = DirectoryIter::new(&buffer);
        let entry = directory_iter.next().unwrap();
        print!("Location: {:0x}", entry.location);
    }
}

pub struct DirectoryEntry {
    location: u32, // id: [u8; 256],
}

pub struct DirectoryIter<'a> {
    cur: usize,
    buffer: &'a [u8],
}

impl<'a> DirectoryIter<'a> {
    pub fn new(buffer: &[u8]) -> DirectoryIter {
        DirectoryIter {
            cur: 0,
            buffer: buffer,
        }
    }
}

impl<'a> Iterator for DirectoryIter<'a> {
    type Item = DirectoryEntry;
    fn next(&mut self) -> Option<DirectoryEntry> {
        // let len = self.buffer[0];

        // 		let location = (self.buffer[2 + 3] as u32) << 24 | (self.buffer[2 + 2] as u32) << 16 |
        //                       (self.buffer[2 + 1] as u32) << 8 |
        //                       (self.buffer[2] as u32);

        // let mut id = [0u8; 256];

        // for i in 33..len {
        // 	id[i as usize] = self.buffer[i as usize + 33];
        // }

        // self.cur += len as usize;
        Some(DirectoryEntry {
            location: 0,
//            id: id,
        })
    }
}
