//! Permission model.

#[derive(Debug, Clone, Copy)]
pub struct Mode(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct Permission {
    pub user_read: bool,
    pub user_write: bool,
    pub user_exec: bool,
    pub group_read: bool,
    pub group_write: bool,
    pub group_exec: bool,
    pub other_read: bool,
    pub other_write: bool,
    pub other_exec: bool,
}

impl Permission {
    pub fn from_mode(m: Mode) -> Self {
        let bits = m.0;
        Self {
            user_read: bits & 0o400 != 0,
            user_write: bits & 0o200 != 0,
            user_exec: bits & 0o100 != 0,
            group_read: bits & 0o040 != 0,
            group_write: bits & 0o020 != 0,
            group_exec: bits & 0o010 != 0,
            other_read: bits & 0o004 != 0,
            other_write: bits & 0o002 != 0,
            other_exec: bits & 0o001 != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_mode() {
        let p = Permission::from_mode(Mode(0o644));
        assert!(p.user_read);
        assert!(p.user_write);
        assert!(!p.user_exec);
    }
}
