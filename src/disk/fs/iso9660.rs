use disk::Disk;

pub const PVD_DIR: usize = 156;

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

    pub fn test(&self) {
        let size = self.pvd[PVD_DIR];
        let location = ((self.pvd[PVD_DIR + 5] as u32) << 24 |
                        (self.pvd[PVD_DIR + 4] as u32) << 16 |
                        (self.pvd[PVD_DIR + 3] as u32) << 8 |
                        (self.pvd[PVD_DIR + 2] as u32)) * 4;
        print!("Root Record: Location: {}, Size: {}\n", location, size);

        let mut record_iter = RecordIter::new(&self.disk, location as usize);
        let record = record_iter.next().unwrap();
        print!("Location: {}, Size: {}\n", record.location, record.size);
    }
}

struct Record {
    location: usize,
    size: usize,
}

struct RecordIter<'a> {
    cur: usize,
    location: usize,
	buffer: [u8; 512],
	disk: &'a Disk,
}

impl<'a> RecordIter<'a> {
    pub fn new(disk: &'a Disk, location: usize) -> Self {
		let mut buffer = [0u8; 512];
		disk.read(location as u64, 1, &mut buffer);

        RecordIter {
            cur: 0,
            location: location,
			buffer: buffer,
			disk: disk,
        }
    }
}

impl<'a> Iterator for RecordIter<'a> {
    type Item = Record;
    fn next(&mut self) -> Option<Record> {
		if self.cur >= 512 {
			self.location += 1;
			self.disk.read(self.location as u64, 1, &mut self.buffer);
		}

        Some(Record {
            location: 0,
            size: 0,
        })
    }
}
