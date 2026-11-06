//! Mount table model.
use crate::vfs::FileSystem;

pub struct Mount {
    pub mountpoint: String,
    pub fs: Box<dyn FileSystem>,
}

#[derive(Default)]
pub struct MountTable {
    pub mounts: Vec<Mount>,
}

impl MountTable {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, m: Mount) { self.mounts.push(m); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mount_table() {
        let mut mt = MountTable::new();
        assert_eq!(mt.mounts.len(), 0);
    }
}
