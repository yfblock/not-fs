#[macro_use]
extern crate alloc;

use alloc::rc::Rc;

use not_fs_core::dentry::FileOps;

pub struct RamFile {
    name: String,

}

impl FileOps for RamFile {
    fn create_node(&self, name: &str) -> Rc<dyn FileOps> {
        todo!()
    }

    fn delete_node(&self, name: &str) {
        todo!()
    }

    fn get_attribute(&self) -> u32 {
        todo!()
    }

    fn set_attribute(&self, attr: u32) {
        todo!()
    }

    fn get_time(&self, ) -> [not_fs_core::dentry::TimeSepc; 3] {
        todo!()
    }

    fn set_time(&self, atime: Option<not_fs_core::dentry::TimeSepc>,
        ctime: Option<not_fs_core::dentry::TimeSepc>, mtime: Option<not_fs_core::dentry::TimeSepc>) {
        todo!()
    }

    fn rename(&self) {
        todo!()
    }

    fn write(&self, data: &[u8]) {
        todo!()
    }

    fn read(&self) -> &[u8] {
        todo!()
    }

    fn file_size(&self) -> usize {
        todo!()
    }
}

