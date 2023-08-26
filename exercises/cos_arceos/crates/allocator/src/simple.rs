//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
pub use core::num::NonZeroUsize;

use crate::{AllocResult, AllocError, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    allocations: usize,
    start: usize,
    end: usize,
    next: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            allocations: 0,
            start: 0,
            end: 0,
            next: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, _start: usize, _size: usize) {
        self.start = _start;
        self.end = self.start + _size;
        self.next = _start;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        self.end = self.end + _size;
        Ok(())
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, _layout: Layout) -> AllocResult<NonZeroUsize> {
        let old = self.next;
        let new = self.next + _layout.size();
        if new > self.end {
            return Err(AllocError::NoMemory)
        }
        self.next = new;
        self.allocations += 1;
        NonZeroUsize::new(old).ok_or(AllocError::NotAllocated)
    }

    fn dealloc(&mut self, _pos: NonZeroUsize, _layout: Layout) {
        self.allocations -= 1;
        if self.allocations == 0 {
            self.next = self.start;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end
    }

    fn used_bytes(&self) -> usize {
        self.next
    }

    fn available_bytes(&self) -> usize {
        self.end - self.next
    }
}
