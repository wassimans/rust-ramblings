use std::alloc;
use std::alloc::Layout;
use std::ptr::NonNull;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        MyVec {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    fn capacity(&self) -> usize {
        self.capacity
    }

    fn len(&self) -> usize {
        self.len
    }

    fn push(&mut self, item: T) {
        // alloc requires that layout points to some memory of non-zer of size
        // For now lets panic! if it's zero sized
        assert_ne!(std::mem::size_of::<T>(), 0, "No zero sized types");
        if self.capacity == 0 {
            let layout = Layout::array::<T>(4).expect("Could not allocate!");
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Could not allocate memory");
            // at this point we really have a non-null pointer that points to a heap memory of
            // size 4 times T. Meaning that we've allocated enough memory for an item and 3 more
            self.ptr = ptr;
            // Write the pushed item to the allocated heap memory
            // Using 'write' is important because previous memory that ptr points to
            // is not read. If it's read in our case it will be undefined behavior
            // because we're reading an uninitialized memory
            unsafe { ptr.as_ptr().write(item) };
            // Update the vector properties
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            unsafe {
                let offset = self
                    .len
                    .checked_mul(std::mem::size_of::<T>())
                    .expect("Cannot reach memory location");
                assert!(offset < isize::MAX as usize, "Wrapped isize");
                // We know at this point that offset cannot wrap around
                // and pointer points to valid memory and writing to to an offset at
                // self.len is valid
                self.ptr.as_ptr().add(self.len).write(item);
                self.len += 1;
            }
        } else {
            debug_assert!(self.len == self.capacity);
            let new_capacity = self.capacity.checked_mul(2).expect("Capacity wrapped");
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;

            size.checked_add(size % align).expect("Can't allocate");

            let ptr = unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not reallocate memory");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            };
            self.ptr = ptr;
            self.len += 1;
            self.capacity = new_capacity;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec: MyVec<usize> = MyVec::new();
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.len(), 3);
    }
}
