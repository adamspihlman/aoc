#[derive(Debug, PartialEq, Copy, Clone)]
enum DiskLocation {
    File(u64),
    Blank,
}

#[derive(Debug)]
pub struct Disk {}

impl Disk {
    pub fn from(input: &str) -> Disk {
        Disk {}
    }

    pub fn compute_checksum(&self) -> u64 {
        0
    }
}
