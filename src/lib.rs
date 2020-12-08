#![no_std]
#![allow(unused_unsafe)]
#![cfg_attr(feature = "nightly", feature(alloc_layout_extra))]

extern crate alloc as std;

use core::{
    alloc::Layout,
    marker::PhantomData,
    mem::{size_of, MaybeUninit},
};

#[cfg(feature = "nightly")]
mod nightly;
#[cfg(feature = "nightly")]
use nightly as imp;

#[cfg(not(feature = "nightly"))]
mod stable;
#[cfg(not(feature = "nightly"))]
use stable as imp;

pub struct SmallVec<T, A> {
    capacity: usize,
    data: imp::Data<T, A>,
}

#[repr(C)]
struct Inline<T, A> {
    align: [T; 0],
    array: MaybeUninit<A>,
}

pub(crate) struct Heap<T> {
    ptr: *mut T,
    len: usize,
    drop: PhantomData<T>,
}

impl<T, A> Drop for SmallVec<T, A> {
    fn drop(&mut self) {
        struct DeallocOnDrop<'a, T, A>(&'a mut SmallVec<T, A>);

        impl<'a, T, A> Drop for DeallocOnDrop<'_, T, A> {
            fn drop(&mut self) {
                unsafe {
                    self.0.data.dealloc(self.0.capacity());
                }
            }
        }

        unsafe {
            let dealloc = DeallocOnDrop(self);
            let this = &mut *dealloc.0;
            let (ptr, len, _) = this.raw_parts_mut();
            core::ptr::slice_from_raw_parts_mut(ptr, *len).drop_in_place()
        }
    }
}

impl<T> Heap<T> {
    unsafe fn dealloc(&mut self, layout: Layout) { std::alloc::dealloc(self.ptr.cast(), layout) }
}

impl<T, A> Inline<T, A> {
    pub const CAPACITY: usize = if size_of::<T>() == 0 {
        usize::MAX
    } else {
        size_of::<A>() / size_of::<T>()
    };

    const fn new() -> Self {
        Inline {
            array: MaybeUninit::uninit(),
            align: [],
        }
    }
}

impl<T, A> SmallVec<T, A> {
    pub const INLINE_CAPACITY: usize = Inline::<T, A>::CAPACITY;

    pub fn new() -> Self {
        Self {
            data: imp::Data::new(),
            capacity: 0,
        }
    }

    pub fn capacity(&self) -> usize { self.capacity.max(Self::INLINE_CAPACITY) }

    pub fn is_inline(&self) -> bool { self.capacity <= Self::INLINE_CAPACITY }

    pub fn len(&self) -> usize {
        #[cfg(feature = "nightly")]
        {
            unsafe { self.data.len(self.capacity, self.capacity <= Self::INLINE_CAPACITY) }
        }

        #[cfg(not(feature = "nightly"))]
        {
            self.data.len(self.capacity)
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.reserve_capacity(self.len().checked_add(additional).expect("Overflow when reserving"))
    }

    fn reserve_capacity(&mut self, new_capacity: usize) {
        let old_capacity = self.capacity();
        if new_capacity > old_capacity {
            #[cfg(feature = "nightly")]
            unsafe {
                self.data
                    .reserve(old_capacity, new_capacity, old_capacity <= Self::INLINE_CAPACITY);
            }

            #[cfg(not(feature = "nightly"))]
            {
                unsafe { self.data.reserve(old_capacity, new_capacity) };
            }

            self.capacity = new_capacity;
        }
    }

    pub fn as_ptr(&self) -> *const T {
        #[cfg(feature = "nightly")]
        unsafe {
            self.data.as_ptr(self.is_inline())
        }

        #[cfg(not(feature = "nightly"))]
        {
            self.data.as_ptr()
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        #[cfg(feature = "nightly")]
        unsafe {
            self.data.as_mut_ptr(self.is_inline())
        }

        #[cfg(not(feature = "nightly"))]
        {
            self.data.as_mut_ptr()
        }
    }

    unsafe fn raw_parts(&self) -> (*const T, usize, usize) {
        let cap = self.capacity();
        #[cfg(not(feature = "nightly"))]
        let (ptr, len) = self.data.raw_parts();

        #[cfg(feature = "nightly")]
        let (ptr, len) = self.data.raw_parts(self.is_inline());

        (ptr, len.unwrap_or(self.capacity), cap)
    }

    unsafe fn raw_parts_mut(&mut self) -> (*mut T, &mut usize, usize) {
        let cap = self.capacity();
        #[cfg(not(feature = "nightly"))]
        let (ptr, len_ref) = self.data.raw_parts_mut();

        #[cfg(feature = "nightly")]
        let (ptr, len_ref) = self.data.raw_parts_mut(self.is_inline());

        (ptr, len_ref.unwrap_or(&mut self.capacity), cap)
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe {
            let (ptr, len, _) = self.raw_parts();
            core::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe {
            let (ptr, len, _) = self.raw_parts_mut();
            core::slice::from_raw_parts_mut(ptr, *len)
        }
    }

    pub fn push(&mut self, value: T) -> &mut T {
        let len = self.len();
        if len == self.capacity() {
            self.reserve(len + 1);
        }

        unsafe {
            let (ptr, len, _) = self.raw_parts_mut();
            let ptr = ptr.add(*len);
            *len += 1;
            ptr.write(value);
            &mut *ptr
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            let (ptr, len_ref, _) = self.raw_parts_mut();
            let len = len_ref.checked_sub(1)?;
            *len_ref = len;

            let ptr = ptr.add(len);
            Some(ptr.read())
        }
    }
}
