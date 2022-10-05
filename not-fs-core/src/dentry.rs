use alloc::vec::Vec;
use alloc::rc::{Weak, Rc};
use alloc::string::String;

use crate::mount::Mount;

#[derive(Clone, Copy)]
pub struct TimeSepc {
    tv_sec: u64,
    tv_nsec: u64
}

pub trait FileOps {
    fn create_node(&self, name: &str) -> Rc<dyn FileOps>;
    fn delete_node(&self, name: &str);
    fn get_attribute(&self) -> u32;
    fn set_attribute(&self, attr: u32);
    fn get_time(&self, ) -> [TimeSepc; 3];
    fn set_time(&self, atime: Option<TimeSepc>,
        ctime: Option<TimeSepc>, mtime: Option<TimeSepc>);
    fn rename(&self);
    fn write(&self, data: &[u8]);
    fn read(&self) -> &[u8];
    fn file_size(&self) -> usize;
}

/// FileTree Node
/// 
/// This is a FileTree node. Storing the node tree to memory
/// can improve the reading speed.
#[derive(Clone)]
pub struct Dentry {
    pub refcnt: u32,                    // the count of reference
    pub name: String,                   // filename
    pub mount: Option<Rc<Mount>>,       // mount data
    pub parent: Weak<Dentry>,           // parent dentry
    pub child: Vec<Rc<Dentry>>,         // child dentry
    pub file: Option<Rc<dyn FileOps>>,  // file use ops
}

impl Dentry {
    /// Umount
    /// 
    /// Umount filesystem, will called by mount
    pub fn umount(&mut self) {
        self.mount = None;
        self.child.clear();
    }
}

unsafe impl Sync for Dentry {}

impl Dentry {
    /// Get child node
    pub fn get_node(&self, name: &str) -> Option<Rc<Self>> {
        for child_node in &self.child {
            if child_node.name == name {
                return Some(child_node.clone());
            }
        }
        None
    }

    /// Get mount info
    /// 
    /// Get mount info of this dentry node.
    pub fn get_mount(&self) -> Option<Rc<Mount>> {
        self.mount.clone()
    }

    /// Create Child Node
    /// 
    /// Create a blank child node
    pub fn create_child(self: Rc<Self>, name: &str) -> Option<Rc<Self>> {
        self.file.clone().unwrap().create_node(name);
        Some(Rc::new(Dentry {
            refcnt: 0,
            name: String::from(name),
            mount: self.get_mount(),
            parent: Rc::downgrade(&self),
            child: vec![],
            file: self.file.clone(),
        }))
    }
}