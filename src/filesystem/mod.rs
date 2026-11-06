// SigmaOS Filesystem Module — minimal working set
pub mod core;
pub mod vfs;
pub mod manager;

pub use vfs::{FileType, FsError, Inode, VfsError, VirtualFilesystem};
pub use core::*;
pub use manager::*;
