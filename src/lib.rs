#![no_std]
#![allow(unused_unsafe)]
#![feature(alloc_layout_extra)]

extern crate alloc as std;

use core::{
    alloc::Layout,
    marker::PhantomData,
    mem::{size_of, MaybeUninit},
};

mod nightly {}
use nightly as imp;

pub struct SmallVec;

#[repr(C)]
struct Inline;

pub(crate) struct Heap;

impl Drop for SmallVec {
    fn drop(&mut self) {
        struct DeallocOnDrop;

        impl Drop for DeallocOnDrop {
            fn drop(&mut self) { loop {} }
        }

        loop {}
    }
}

impl Heap {
    unsafe fn dealloc(&mut self, layout: Layout) { loop {} }
}

impl Inline {
    pub const CAPACITY: usize = 0;

    const fn new() -> Self { loop {} }
}

impl SmallVec {
    pub const INLINE_CAPACITY: usize = 0;

    pub fn new() -> Self { loop {} }

    pub fn capacity(&self) -> usize { loop {} }

    pub fn is_inline(&self) -> bool { loop {} }

    pub fn len(&self) -> usize { loop {} }

    pub fn reserve(&mut self, additional: usize) { loop {} }

    fn reserve_capacity(&mut self, new_capacity: usize) { loop {} }

    pub fn as_ptr(&self) -> *const () { loop {} }

    pub fn as_mut_ptr(&mut self) -> *mut () { loop {} }

    unsafe fn raw_parts(&self) -> (*const (), usize, usize) { loop {} }

    unsafe fn raw_parts_mut(&mut self) -> (*mut (), &mut usize, usize) { loop {} }

    pub fn as_slice(&self) -> &[()] { loop {} }

    pub fn as_mut_slice(&mut self) -> &mut [()] { loop {} }

    pub fn push(&mut self, value: ()) -> &mut () { loop {} }

    pub fn pop(&mut self) -> Option<()> { loop {} }
}
