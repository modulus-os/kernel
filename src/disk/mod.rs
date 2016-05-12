/// ATA specific disk driver
pub mod ata;

/// A storage disk
///
/// This is a trait implemented by specific disk types (ATA, ATAPI, SATA) for transferring data
/// between a disk and memory.
pub trait Disk {
    /// Read from disk
    ///
    /// Read content from the disk at specified block. To read multiple blocks/sectors, change the
    /// `count` parameter to the chosen number of blocks. The `buffer` should be an array of `u8`s
    /// of size `count * 512`. This is where to received data will be written.
    fn read(&self, block: u64, count: u16, buffer: &mut [u8]);
    /// Write to disk
    ///
    /// Write content to the disk at specified block. To write multiple blocks/sectors, change the
    /// `count` parameter to the chosen number of blocks. The `buffer` should be an array of `u8`s
    /// of size `count * 512`. This is where the data to be written will be located.
    fn write(&self, block: u64, count: u16, buffer: &[u8]);
}
