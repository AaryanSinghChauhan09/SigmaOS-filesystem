//! SigmaOS-filesystem — filesystem model.
//!
//! Models VFS (virtual filesystem), inodes, directory entries,
//! file descriptors, permissions, and basic mount/dentry structures.
//! Deterministic and std-only (no actual syscalls).

pub mod vfs;
pub mod inode;
pub mod dentry;
pub mod mount;
pub mod fd;
pub mod perm;

pub use vfs::{Vfs, FileSystem, FileType, VfsError};
pub use inode::{Inode, InodeKind, InodeMode};
pub use dentry::{Dentry, DentryCache};
pub use mount::{Mount, MountTable};
pub use fd::{FileDescriptor, FdTable, OpenFlags};
pub use perm::{Permission, Mode};
