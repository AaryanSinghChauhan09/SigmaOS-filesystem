//! Inode model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InodeKind { File, Directory, Symlink }

#[derive(Debug, Clone, Copy)]
pub struct InodeMode {
    pub mode: u32,
}

#[derive(Debug, Clone)]
pub struct Inode {
    pub ino: u64,
    pub kind: InodeKind,
    pub mode: InodeMode,
    pub size: u64,
    pub nlink: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inode_kind() {
        let i = Inode { ino: 1, kind: InodeKind::Directory, mode: InodeMode { mode: 0o755 }, size: 0, nlink: 2 };
        assert_eq!(i.kind, InodeKind::Directory);
    }
}
