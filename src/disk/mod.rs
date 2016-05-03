/// ATA disk driver
pub mod ata;

pub trait Disk {
    fn read(&self, block: u64, count: u16, buffer: *mut u16);
    fn write(&self, block: u64, count: u16, buffer: *mut u16);
}
