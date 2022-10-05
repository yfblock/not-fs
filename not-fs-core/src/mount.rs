use core::cell::RefCell;

use alloc::rc::{Rc, Weak};

use crate::dentry::Dentry;

pub struct MountInner {
    dentry: Weak<Dentry>
}

pub struct Mount(Rc<RefCell<MountInner>>);


impl Mount {
    
}