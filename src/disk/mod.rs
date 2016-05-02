/// ATA disk driver
pub mod ata;

pub trait Disk {
    fn read(&self, lba: u64, count: u8, buffer: *mut u16);
}
