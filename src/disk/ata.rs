use io::pio::*;
use disk::Disk;

const COMMAND: u16 = 0x7;
const SELECTOR: u16 = 0x6;
const DATA: u16 = 0x0;

pub struct Ata {
    base: u16,
    pub slave: bool,
}

impl Disk for Ata {
    /// Read 48
    ///
    /// Read from disk using a 48-bit LBA (address) and write contents into buffer
    fn read(&self, block: u64, count: u16, buffer: *mut u16) {
        let selector;
        if self.slave {
            selector = 0x50;
        } else {
            selector = 0x40;
        }

        outb(self.base + SELECTOR, selector);
        outb(self.base + 1, 0);

        self.lba48(block, count);

        // Send READ SECTORS EXT command
        outb(self.base + COMMAND, 0x24);

        // Read a each sector
        for c in 0..count {
            // Poll until ready
            while !self.poll() {}

            // Read sector
            for i in 0..256 {
                let data = inw(self.base + DATA);
                unsafe {
                    *buffer.offset((c as isize * 256) + i as isize) = data;
                }
            }

            // 400ns delay
            self.delay_400ns();
        }
    }

    /// Write 48
    ///
    /// Write contents of buffer to disk using a 48-bit LBA (address)
    fn write(&self, block: u64, count: u16, buffer: *mut u16) {
        let selector;
        if self.slave {
            selector = 0x50;
        } else {
            selector = 0x40;
        }

        outb(self.base + SELECTOR, selector);
        outb(self.base + 1, 0);

        self.lba48(block, count);

        // Send WRITE SECTORS EXT command
        outb(self.base + COMMAND, 0x34);

        for i in 0..count {
            // Poll until ready
            while !self.poll() {}

            // Write
            for j in 0..256 {
                outw(self.base + DATA,
                     unsafe { *buffer.offset((i as isize * 256) + j) });
                // for _ in 0..1000 {}
            }

            // 400ns delay
            self.delay_400ns();

            // Cache flush
            outb(self.base + COMMAND, 0xe7);
        }

        let mut status = inb(self.base + COMMAND);
        while (status & 0b10000000) != 0 {
            status = inb(self.base + COMMAND);
        }
        while (status & 0b00001000) == 0 {
            status = inb(self.base + COMMAND);
        }
    }
}

impl Ata {
    pub fn new(base: u16, slave: bool) -> Option<Self> {
        let disk = Ata {
            base: base,
            slave: slave,
        };

        // Make sure that disk exists
        let identity = disk.identify();

        if identity.is_none() {
            None
        } else {
            Some(disk)
        }
    }

    /// IDENTIFY command
    ///
    /// Returns information about a drive if it exists.
    pub fn identify(&self) -> Option<u8> {
        let command = self.base + COMMAND;
        let selector = self.base + SELECTOR;
        let data = self.base + DATA;

        // Select drive
        if self.slave {
            outb(selector, 0xb0);
        } else {
            outb(selector, 0xa0);
        }

        outb(self.base + 2, 0);
        outb(self.base + 3, 0);
        outb(self.base + 4, 0);
        outb(self.base + 5, 0);

        // Send IDENTIFY command
        outb(command, 0xec);

        // Read status
        let mut status = inb(command);

        if status == 0 {
            return None;
        } else {
            while (status & 0b10000000) != 0 {
                status = inb(command);
            }

            // Check if drive is ATA
            if inb(0x1f4) != 0 {
                return None;
            }
            if inb(0x1f5) != 0 {
                return None;
            }

            while (status & 0b00001111) == 0 {
                status = inb(command);
            }

            // Check if ERR is set
            if (status & 0b00000111) != 0 {
                return None;
            }

            return Some(inb(data));
        }
    }

    pub fn read28(&self, lba: u32, count: u8, buffer: *mut u16) {
        let selector;
        if self.slave {
            selector = 0xf0;
        } else {
            selector = 0xe0;
        }

        outb(self.base + SELECTOR, selector | ((lba >> 24) & 0xf0) as u8);
        outb(self.base + 1, 0);
        outb(self.base + 2, count);
        outb(self.base + 3, lba as u8);
        outb(self.base + 4, (lba >> 8) as u8);
        outb(self.base + 5, (lba >> 16) as u8);
        // Send READ SECTORS command
        outb(self.base + COMMAND, 0x20);

        // Wait until BSY is cleared
        for _ in 0..count {
            let mut status = inb(self.base + COMMAND);
            while (status & 0b10000000) != 0 {
                status = inb(self.base + COMMAND);
            }
            while (status & 0b00001000) == 0 {
                status = inb(self.base + COMMAND);
            }

            // Read sector
            for i in 0..255 {
                let data = inw(self.base + DATA);
                unsafe {
                    *buffer.offset(i) = data;
                }
                print!("{:0x}", unsafe { *buffer.offset(i) });
            }

            print!(" : ");
        }
    }

    fn lba48(&self, lba: u64, count: u16) {
        let lba1 = (lba & 0x0000000000ff) as u8;
        let lba2 = ((lba & 0x00000000ff00) >> 8) as u8;
        let lba3 = ((lba & 0x000000ff0000) >> 16) as u8;
        let lba4 = ((lba & 0x0000ff000000) >> 24) as u8;
        let lba5 = ((lba & 0x00ff00000000) >> 32) as u8;
        let lba6 = ((lba & 0xff0000000000) >> 40) as u8;

        outb(self.base + 2, (count & 0xff00) as u8);
        outb(self.base + 3, lba4);
        outb(self.base + 4, lba5);
        outb(self.base + 5, lba6);

        outb(self.base + 2, (count & 0x00ff) as u8);
        outb(self.base + 3, lba1);
        outb(self.base + 4, lba2);
        outb(self.base + 5, lba3);
    }

    fn delay_400ns(&self) {
        for _ in 0..4 {
            inb(self.base + 0x200 + 0x6);
        }
    }

    /// Returns true if device is ready
    fn poll(&self) -> bool {
        let status = inb(self.base + COMMAND);
        if status & 0b10000000 == 0 {
            if status & 0b00001000 != 0 || status & 0b00100001 != 0 {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}