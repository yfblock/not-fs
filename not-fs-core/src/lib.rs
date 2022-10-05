/*
 * THIS IS A RUST CRATE FOR FILESYSTEM INTERFACE, IF YOU NEED
 * TO IMPLEMENT THE FILESYSTEM IN YOUR CUSTOM OS. YOU CAN CONSIDER
 * USE THIS CRATE IN YOUR OS. THIS CRATE PROVIDE THE EASY BUT 
 * STRONG FUNCTION FOR YOU OS. YOU JUST NEED TO FOCUS ON YOUR
 * OS AND NOT FILESYSTEM.
 * 
 * AUTHOR: YUFENG
 * EMAIL: 321353225@qq.com
 * GITHUB ID: yfblock
 */

// #![no_std]

pub mod dentry;
mod mount;

#[macro_use]
extern crate alloc;

use alloc::string::String;

pub trait FSOps {
    fn mount(path: String);
}

pub fn init() {

}