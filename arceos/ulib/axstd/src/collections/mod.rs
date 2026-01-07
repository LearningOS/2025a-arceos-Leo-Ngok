extern crate alloc;
#[cfg(feature = "alloc")]
#[doc(no_inline)]
pub use alloc::{boxed, collections, format, string, vec};

use alloc::vec::Vec;
use alloc::string::String;

pub struct HashMap {
    keys: Vec<String>,
    values: Vec<u32>,
    iter_offset: usize,
}

impl HashMap {
    pub fn new() -> Self {
        Self {
            keys : vec![], values: vec![], iter_offset : 0
        }
    }
    pub fn insert(&mut self, _k: String, _v: u32) {
        self.keys.push(_k);
        self.values.push(_v);
    }
    pub fn iter(&mut self) -> Option<(String, &u32)> {
        if self.iter_offset == self.keys.len() {
            None
        } else {
            let ret = (self.keys[self.iter_offset].clone(),
                &self.values[self.iter_offset]);
            self.iter_offset += 1;
            Some(ret)
        }
        
    }
}