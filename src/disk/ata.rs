use io::pio::*;
use disk::Disk;

const DATA: u16 = 0x0;
const SECTOR_COUNT: u16 = 0x2;
const LBA_LOW: u16 = 0x3;
const LBA_MID: u16 = 0x4;
const LBA_HIGH: u16 = 0x5;
const SELECTOR: u16 = 0x6;
const COMMAND: u16 = 0x7;

pub struct Ata {
    base: u16,
    pub slave: bool,
}

pub fn list() {
    let pm = Ata::new(0x1f0, false).identify();
    let ps = Ata::new(0x1f0, true).identify();

    print!("Primary Master: \n");
    if let Some(info) = pm {
        print!("   Serial: ");
        for i in 20..40 {
            print!("{}", info[i] as char);
        }
        print!("\n   Firmware: ");
        for i in 46..54 {
            print!("{}", info[i] as char);
        }
        print!("\n   Model: ");
        for i in 54..94 {
            print!("{}", info[i] as char);
        }
        print!("\n");
    } else {
        print!("   No disk found\n");
    }

    print!("Primary Slave: \n");
    if let Some(info) = ps {
        print!("   Serial: ");
        for i in 20..40 {
            print!("{}", info[i] as char);
        }
        print!("\n   Firmware: ");
        for i in 46..54 {
            print!("{}", info[i] as char);
        }
        print!("\n   Model: ");
        for i in 54..94 {
            print!("{}", info[i] as char);
        }
        print!("\n");
    } else {
        print!("   No disk found\n");
    }
}

impl Disk for Ata {
    /// Read 48
    ///
    /// Read from disk using a 48-bit LBA (address) and write contents into buffer
    fn read(&self, block: u64, count: u16, buffer: &mut [u8]) {
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
                if buffer.len() >= (c as usize * 256) {
                    buffer[i * 2 + (c as usize * 256)] = data as u8;
                    buffer[i * 2 + 1 + (c as usize * 256)] = (data >> 8) as u8;
                }
            }

            // 400ns delay
            self.delay_400ns();
        }

    }

    /// Write 48
    ///
    /// Write contents of buffer to disk using a 48-bit LBA (address)
    fn write(&self, block: u64, count: u16, buffer: &[u8]) {
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

        for c in 0..count {
            // Poll until ready
            while !self.poll() {}

            // Write
            for i in 0..256 {
                outw(self.base + DATA,
                     (buffer[i + (c as usize * 256)] as u16) << 8 |
                     buffer[i + 1 + (c as usize * 256)] as u16);
            }

            // 400ns delay
            self.delay_400ns();

            // Cache flush
            outb(self.base + COMMAND, 0xe7);
        }
    }
}

impl Ata {
    pub fn new(base: u16, slave: bool) -> Self {
        Ata {
            base: base,
            slave: slave,
        }
    }

    /// IDENTIFY command
    ///
    /// Returns information about a drive if it exists.
    pub fn identify(&self) -> Option<[u8; 512]> {
        let command = self.base + COMMAND;
        let selector = self.base + SELECTOR;

        // Select drive
        if self.slave {
            outb(selector, 0xb0);
        } else {
            outb(selector, 0xa0);
        }

        outb(self.base + SECTOR_COUNT, 0);
        outb(self.base + LBA_LOW, 0);
        outb(self.base + LBA_MID, 0);
        outb(self.base + LBA_HIGH, 0);

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
            if inb(self.base + LBA_MID) != 0 {
                return None;
            }

            if inb(self.base + LBA_HIGH) != 0 {
                return None;
            }

            while (status & 0b00001111) == 0 {
                status = inb(command);
            }

            // Check if ERR is set
            if (status & 0b00000111) != 0 {
                return None;
            }

            let mut buffer = [0u8; 512];

            for i in 0..256 {
                let data = inw(self.base + DATA);
                buffer[i * 2] = (data >> 8) as u8;
                buffer[i * 2 + 1] = data as u8;
            }

            Some(buffer)
        }
    }

    fn lba48(&self, lba: u64, count: u16) {
        let lba1 = (lba & 0x0000000000ff) as u8;
        let lba2 = ((lba & 0x00000000ff00) >> 8) as u8;
        let lba3 = ((lba & 0x000000ff0000) >> 16) as u8;
        let lba4 = ((lba & 0x0000ff000000) >> 24) as u8;
        let lba5 = ((lba & 0x00ff00000000) >> 32) as u8;
        let lba6 = ((lba & 0xff0000000000) >> 40) as u8;

        outb(self.base + SECTOR_COUNT, (count & 0xff00) as u8);
        outb(self.base + LBA_LOW, lba4);
        outb(self.base + LBA_MID, lba5);
        outb(self.base + LBA_HIGH, lba6);

        outb(self.base + SECTOR_COUNT, (count & 0x00ff) as u8);
        outb(self.base + LBA_LOW, lba1);
        outb(self.base + LBA_MID, lba2);
        outb(self.base + LBA_HIGH, lba3);
    }

    fn delay_400ns(&self) {
        for _ in 0..4 {
            inb(self.base + 0x200 + 0x6);
        }
    }

    /// Returns true if device is ready
    fn poll(&self) -> bool {
        let status = inb(self.base + COMMAND);

        if status & 0x80 == 0x80 {
            return false;
        } else {
            return true;
        }
    }
}
