//! Dentry cache model.

#[derive(Debug, Clone)]
pub struct Dentry {
    pub name: String,
    pub inode: u64,
    pub parent: Option<u64>,
}

#[derive(Debug, Default)]
pub struct DentryCache {
    entries: Vec<Dentry>,
}

impl DentryCache {
    pub fn new() -> Self { Self::default() }
    pub fn insert(&mut self, d: Dentry) { self.entries.push(d); }
    pub fn lookup(&self, name: &str) -> Option<&Dentry> {
        self.entries.iter().find(|d| d.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dentry_cache() {
        let mut dc = DentryCache::new();
        dc.insert(Dentry { name: "test".into(), inode: 1, parent: None });
        assert!(dc.lookup("test").is_some());
    }
}
