#![no_std]

extern crate log;

use core::u8;

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};
use log::{debug, warn};

// use core::fmt;
// use 
/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const SIZE: usize> {
    loc_start : usize,
    bytes_end: usize,
    loc_size : usize
}

impl<const SIZE: usize> EarlyAllocator<SIZE> {
    pub const fn new() -> Self {
        Self {loc_size: 0, loc_start: 0, bytes_end : 0}
    }
}
// static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator::new();

// /// Returns the reference to the global allocator.
// pub fn global_allocator() -> &'static GlobalAllocator {
//     &GLOBAL_ALLOCATOR
// }
static mut ADDR_START : usize = 0;

impl<const SIZE: usize> BaseAllocator for EarlyAllocator<SIZE> {
    // let mut loc_start : usize = 0;
    fn init(&mut self, start: usize, size: usize) {
        // todo!()
        // println!("start = {}, size = {}", start, size);
        // panic!("start = {}, size = {}", start, size);
        // debug_assert!()
        // log_syntax!()
        debug!("(Base allocator init) start = {}, size = {}\n", start, size);
        debug!("Base allocator: SIZE = {}\n", SIZE);
        self.loc_start = start;
        self.loc_size = size;
        self.bytes_end = start;
        unsafe {ADDR_START = start;}

    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        // todo!();
        // panic!("start = {}, size = {}", start, size);
        warn!("(Base allocator add memory) start = {}, size = {} yet to be implemented \n", start, size);
        Ok(())
    }
}

impl<const SIZE: usize> ByteAllocator for EarlyAllocator<SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        
        debug!("malloc: layout alignment {}, layout size {}\n", layout.align(), layout.size());
        let start = if self.bytes_end % layout.align() == 0 {
            self.bytes_end
        } else { self.bytes_end - (self.bytes_end % layout.align()) + layout.align()};
        let end = start + layout.size();
        self.bytes_end = end;
        if let Some(start_ptr) = core::ptr::NonNull::new(start as *mut u8) {
            Ok(start_ptr)
        } else {
            Err(allocator::AllocError::NoMemory)
        }
        // Ok(core::ptr::NonNull::new(start as *mut u8).unwrap())
        // todo!()
        // layout.
        // let mut 
        // Ok(())

    }

    fn dealloc(&mut self, pos: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        debug!("free: This implementation does not consider returning mem to allocator, may result in memory leak !!");
        debug!("free: layout alignment {}, layout size {}\n", layout.align(), layout.size());
        // todo!()
    }

    fn total_bytes(&self) -> usize {
        // todo!()
        self.loc_size
    }

    fn used_bytes(&self) -> usize {
        // todo!()
        self.bytes_end - self.loc_start
    }

    fn available_bytes(&self) -> usize {
        // todo!()
        self.loc_start + self.loc_size - self.bytes_end
    }
}

impl<const SIZE: usize> PageAllocator for EarlyAllocator<SIZE> {
    const PAGE_SIZE: usize = SIZE;

    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        todo!()
    }

    fn dealloc_pages(&mut self, pos: usize, num_pages: usize) {
        todo!()
    }

    fn total_pages(&self) -> usize {
        todo!()
    }

    fn used_pages(&self) -> usize {
        todo!()
    }

    fn available_pages(&self) -> usize {
        todo!()
    }
}