//! Virtual filesystem model.
use crate::mount::Mount;
use crate::inode::Inode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType { Regular, Directory, Symlink, CharDevice, BlockDevice, Fifo, Socket }

#[derive(Debug, Clone)]
pub enum VfsError { NotFound, PermissionDenied, NotADirectory, IsDirectory, InvalidOp }

pub trait FileSystem {
    fn lookup(&self, path: &str) -> Result<Inode, VfsError>;
    fn read(&self, inode: u64, offset: u64, buf: &mut [u8]) -> Result<usize, VfsError>;
    fn write(&mut self, inode: u64, offset: u64, data: &[u8]) -> Result<usize, VfsError>;
}

#[derive(Default)]
pub struct Vfs {
    pub mounts: Vec<Mount>,
}

impl Vfs {
    pub fn new() -> Self { Self::default() }
    pub fn mount(&mut self, m: Mount) { self.mounts.push(m); }
    pub fn lookup(&self, path: &str) -> Result<Inode, VfsError> {
        self.mounts.iter()
            .find_map(|m| m.fs.lookup(path.strip_prefix(&m.mountpoint).unwrap_or(path)).ok())
            .ok_or(VfsError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vfs_lookup() {
        let vfs = Vfs::new();
        assert!(vfs.lookup("/nonexistent").is_err());
    }
}
