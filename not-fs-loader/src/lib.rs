use alloc::rc::{Weak, Rc};
use not_fs_core::dentry::Dentry;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate alloc;

pub struct FileTree(Rc<Dentry>);

unsafe impl Sync for FileTree {}

lazy_static! {
    // Define The Root Node
    static ref ROOT: FileTree = FileTree(Rc::new(Dentry {
        refcnt: 1,
        name: String::from("/"),
        mount: None,
        parent: Weak::new(),
        child: vec![],
        file: None
    }));
}

/// Get root node
#[inline]
pub fn get_root() -> Rc<Dentry> {
    return ROOT.0.clone();
}

/// Get Path
#[inline]
pub fn get_path_arr(path: &str) -> Vec<&str> {
    path.split("/").filter(|x| x.trim() != "").collect()
}

/// Get a dentry by relative path and a reference dentry.
pub fn open_rel(reference: Rc<Dentry>, path: &str) -> Option<Rc<Dentry>> {
    let path_arr = get_path_arr(path);

    // return root dentry if path is /
    if path_arr.len() == 0 {
        return Some(get_root());
    }

    let mut node = if path_arr[0] == "/" { get_root() } else { reference };
    // use path arr to find node
    for child_name in &path_arr[1..] {
        // look up child in current node
        let child = node.get_node(child_name);

        // continue if it has the child expected
        if let Some(child) = child {
            node = child;
        } else {
            return None;
        }
    }
    Some(node)
}

/// Get a dentry by absolute path
#[inline]
pub fn open_abs(path: &str) -> Option<Rc<Dentry>> {
    open_rel(ROOT.0.clone(), path)
}

/// Get parent's node
/// 
/// Get the parent node of specified node.
#[inline]
pub fn get_parent(dentry: Dentry) -> Option<Rc<Dentry>> {
    dentry.parent.upgrade()
}

#[test]
fn test_split_path() {
    println!("{:?}", get_path_arr("/"));
    println!("{:?}", get_path_arr(r"/\/"));
    println!("{:?}", get_path_arr("./Cargo.toml"));
    println!("{:?}", get_path_arr("main.rs"));
    println!("{:?}", get_path_arr("/home/yufeng/code/"));
}