//! File descriptor table model.

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OpenFlags: u32 {
        const RDONLY = 0o0;
        const WRONLY = 0o1;
        const RDWR = 0o2;
        const CREAT = 0o100;
        const TRUNC = 0o1000;
        const APPEND = 0o2000;
    }
}

#[derive(Debug, Clone)]
pub struct FileDescriptor {
    pub fd: i32,
    pub inode: u64,
    pub flags: OpenFlags,
    pub offset: u64,
}

#[derive(Debug, Default)]
pub struct FdTable {
    pub fds: Vec<FileDescriptor>,
}

impl FdTable {
    pub fn new() -> Self { Self::default() }
    pub fn allocate(&mut self, fd: FileDescriptor) -> i32 {
        let id = self.fds.len() as i32;
        self.fds.push(fd);
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fd_allocate() {
        let mut ft = FdTable::new();
        let id = ft.allocate(FileDescriptor { fd: 0, inode: 1, flags: OpenFlags::RDONLY, offset: 0 });
        assert_eq!(id, 0);
    }
}
