#![no_std]
#![allow(unused_unsafe)]
#![feature(alloc_layout_extra)]

extern crate alloc as std;

use core::{
    alloc::Layout,
    marker::PhantomData,
    mem::{size_of, MaybeUninit},
};

mod nightly {
    use super::{Heap, Inline};

    use core::mem::ManuallyDrop;

    pub(crate) union Data<T, A> {
        inline: ManuallyDrop<Inline<T, A>>,
    }
}
use nightly as imp;

pub struct SmallVec<T, A> {
    data: nightly::Data<T, A>,
}

#[repr(C)]
struct Inline<T, A> {
    align: [T; 0],
    array: MaybeUninit<A>,
}

pub(crate) struct Heap<T> {
    ptr: *mut T,
}

impl<T, A> Drop for SmallVec<T, A> {
    fn drop(&mut self) {
        struct DeallocOnDrop<'a, T, A>(&'a mut SmallVec<T, A>);

        impl<'a, T, A> Drop for DeallocOnDrop<'_, T, A> {
            fn drop(&mut self) { loop {} }
        }

        loop {}
    }
}

impl<T> Heap<T> {
    unsafe fn dealloc(&mut self, layout: Layout) { loop {} }
}

impl<T, A> Inline<T, A> {
    pub const CAPACITY: usize = 0;

    const fn new() -> Self { loop {} }
}

impl<T, A> SmallVec<T, A> {
    pub const INLINE_CAPACITY: usize = 0;

    pub fn new() -> Self { loop {} }

    pub fn capacity(&self) -> usize { loop {} }

    pub fn is_inline(&self) -> bool { loop {} }

    pub fn len(&self) -> usize { loop {} }

    pub fn reserve(&mut self, additional: usize) { loop {} }

    fn reserve_capacity(&mut self, new_capacity: usize) { loop {} }

    pub fn as_ptr(&self) -> *const T { loop {} }

    pub fn as_mut_ptr(&mut self) -> *mut T { loop {} }

    unsafe fn raw_parts(&self) -> (*const T, usize, usize) { loop {} }

    unsafe fn raw_parts_mut(&mut self) -> (*mut T, &mut usize, usize) { loop {} }

    pub fn as_slice(&self) -> &[T] { loop {} }

    pub fn as_mut_slice(&mut self) -> &mut [T] { loop {} }

    pub fn push(&mut self, value: T) -> &mut T { loop {} }

    pub fn pop(&mut self) -> Option<T> { loop {} }
}
