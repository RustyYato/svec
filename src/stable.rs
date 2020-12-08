use super::{Heap, Inline};

use core::{marker::PhantomData, ptr::NonNull};
use std::alloc::{alloc, handle_alloc_error, realloc, Layout};

mod std_hack;
use std_hack::LayoutExt;

use Data::*;
pub(crate) enum Data<T, A> {
    Inline(Inline<T, A>),
    Heap(Heap<T>),
}

impl<T, A> Data<T, A> {
    pub const fn new() -> Self { Self::Inline(Inline::new()) }
}

impl<T, A> Data<T, A> {
    pub fn as_ptr(&self) -> *const T {
        match self {
            Inline(inline) => inline.array.as_ptr().cast(),
            Heap(heap) => heap.ptr,
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        match self {
            Inline(inline) => inline.array.as_mut_ptr().cast(),
            Heap(heap) => heap.ptr,
        }
    }

    pub fn raw_parts(&self) -> (*const T, Option<usize>) {
        match self {
            Inline(inline) => (inline.array.as_ptr().cast(), None),
            Heap(heap) => (heap.ptr, Some(heap.len)),
        }
    }

    pub fn raw_parts_mut(&mut self) -> (*mut T, Option<&mut usize>) {
        match self {
            Inline(inline) => (inline.array.as_mut_ptr().cast(), None),
            Heap(heap) => (heap.ptr, Some(&mut heap.len)),
        }
    }

    pub fn len(&self, capacity: usize) -> usize {
        match self {
            Inline(_) => capacity,
            Heap(heap) => heap.len,
        }
    }

    #[inline(always)]
    pub unsafe fn reserve(&mut self, old_capacity: usize, new_capacity: usize) {
        self.reserve_exact(old_capacity, new_capacity.max(old_capacity.wrapping_mul(2)).max(4))
    }

    #[cold]
    #[inline(never)]
    pub unsafe fn reserve_exact(&mut self, old_capacity: usize, new_capacity: usize) {
        let layout = Layout::new::<T>()._repeat(new_capacity);

        match self {
            Data::Inline(inline) => {
                let ptr = inline.array.as_ptr().cast::<T>();

                let new_ptr = match NonNull::new(alloc(layout)) {
                    Some(ptr) => ptr.as_ptr().cast::<T>(),
                    None => handle_alloc_error(layout),
                };

                let len = Inline::<T, A>::CAPACITY;
                new_ptr.copy_from_nonoverlapping(ptr, len);

                *self = Self::Heap(Heap {
                    ptr: new_ptr,
                    len,
                    drop: PhantomData,
                })
            }
            Data::Heap(heap) => {
                let old_layout = Layout::new::<T>()._repeat(old_capacity);
                heap.ptr = match NonNull::new(realloc(heap.ptr.cast(), old_layout, layout.size())) {
                    Some(ptr) => ptr.as_ptr().cast(),
                    None => handle_alloc_error(layout),
                };
            }
        }
    }

    pub unsafe fn dealloc(&mut self, capacity: usize) {
        if let Data::Heap(heap) = self {
            heap.dealloc(Layout::new::<T>()._repeat(capacity));
        }
    }
}
