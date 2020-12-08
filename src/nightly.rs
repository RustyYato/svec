use super::{Heap, Inline};

use core::{marker::PhantomData, mem::ManuallyDrop, ptr::NonNull};
use std::alloc::{alloc, handle_alloc_error, realloc, Layout};

pub(crate) union Data<T, A> {
    inline: ManuallyDrop<Inline<T, A>>,
    heap: ManuallyDrop<Heap<T>>,
}

impl<T, A> Data<T, A> {
    pub const fn new() -> Self {
        Self {
            inline: ManuallyDrop::new(Inline::new()),
        }
    }

    pub unsafe fn as_ptr(&self, is_inline: bool) -> *const T {
        if is_inline {
            self.inline.array.as_ptr().cast()
        } else {
            self.heap.ptr
        }
    }

    pub unsafe fn raw_parts(&self, is_inline: bool) -> (*const T, Option<usize>) {
        if is_inline {
            (self.inline.array.as_ptr().cast(), None)
        } else {
            (self.heap.ptr, Some(self.heap.len))
        }
    }

    pub unsafe fn raw_parts_mut(&mut self, is_inline: bool) -> (*mut T, Option<&mut usize>) {
        if is_inline {
            (self.inline.array.as_mut_ptr().cast(), None)
        } else {
            (self.heap.ptr, Some(&mut self.heap.len))
        }
    }

    pub unsafe fn as_mut_ptr(&mut self, is_inline: bool) -> *mut T {
        if is_inline {
            self.inline.array.as_mut_ptr().cast()
        } else {
            self.heap.ptr
        }
    }

    pub unsafe fn len(&self, capacity: usize, is_inline: bool) -> usize {
        if is_inline {
            capacity
        } else {
            self.heap.len
        }
    }

    #[inline(always)]
    pub unsafe fn reserve(&mut self, old_capacity: usize, new_capacity: usize, is_inline: bool) {
        self.reserve_exact(
            old_capacity,
            new_capacity.max(old_capacity.wrapping_mul(2)).max(4),
            is_inline,
        )
    }

    #[cold]
    #[inline(never)]
    pub unsafe fn reserve_exact(&mut self, old_capacity: usize, new_capacity: usize, is_inline: bool) {
        let layout = Layout::new::<T>().repeat(new_capacity).unwrap().0;

        if is_inline {
            let ptr = self.inline.array.as_ptr().cast::<T>();

            let new_ptr = match NonNull::new(alloc(layout)) {
                Some(ptr) => ptr.as_ptr().cast::<T>(),
                None => handle_alloc_error(layout),
            };

            let len = Inline::<T, A>::CAPACITY;
            new_ptr.copy_from_nonoverlapping(ptr, len);

            self.heap = ManuallyDrop::new(Heap {
                ptr: new_ptr,
                len,
                drop: PhantomData,
            });
        } else {
            let old_layout = Layout::new::<T>().repeat(old_capacity).unwrap().0;
            self.heap.ptr = match NonNull::new(realloc(self.heap.ptr.cast(), old_layout, layout.size())) {
                Some(ptr) => ptr.as_ptr().cast(),
                None => handle_alloc_error(layout),
            };
        }
    }

    pub unsafe fn dealloc(&mut self, capacity: usize) {
        self.heap.dealloc(Layout::new::<T>().repeat(capacity).unwrap().0);
    }
}
