#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_slice)]

use std::mem::MaybeUninit;

#[derive(Copy)]
pub struct ArrayVec<T, const CAP: usize> {
    length: u8,
    array: [MaybeUninit<T>; CAP],
}

impl<T: std::fmt::Debug, const CAP: usize> std::fmt::Debug for ArrayVec<T, CAP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        f.write_char('[')?;
        let mut it = self.iter();
        if let Some(first) = it.next() {
            first.fmt(f)?;
        }
        for e in it {
            f.write_str(", ")?;
            e.fmt(f)?;
        }
        f.write_char(']')?;
        Ok(())
    }
}

impl<T, const CAP: usize> std::ops::Deref for ArrayVec<T, CAP> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { MaybeUninit::slice_assume_init_ref(&self.array[0..self.length as usize]) }
    }
}

impl<T, const CAP: usize> std::ops::DerefMut for ArrayVec<T, CAP> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { MaybeUninit::slice_assume_init_mut(&mut self.array[0..self.length as usize]) }
    }
}

impl<T: Clone, const CAP: usize> Clone for ArrayVec<T, CAP> {
    fn clone(&self) -> Self {
        let mut new = Self::new();
        for e in self.iter() {
            new.push(e.clone());
        }
        new
    }
}

impl<T: PartialEq, const CAP: usize> PartialEq for ArrayVec<T, CAP> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        for e in self.iter().zip(other.iter()) {
            if e.0 != e.1 {
                return false;
            }
        }
        true
    }
}

impl<T: Eq, const CAP: usize> Eq for ArrayVec<T, CAP> {}

impl<T, const CAP: usize> ArrayVec<T, CAP> {
    pub fn new() -> Self {
        assert!(CAP <= u8::MAX as usize);
        Self {
            length: 0,
            array: MaybeUninit::uninit_array(),
        }
    }

    pub fn drain_filter<F>(&mut self, filter: F) -> drain_filter::DrainFilter<'_, T, F, CAP>
    where
        F: FnMut(&mut T) -> bool,
    {
        let old_len = self.len() as u8;

        // Guard against us getting leaked (leak amplification)
        unsafe {
            self.set_len(0);
        }

        drain_filter::DrainFilter {
            vec: self,
            idx: 0,
            del: 0,
            old_len,
            pred: filter,
            panic_flag: false,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            unsafe {
                self.length -= 1;
                Some(self.array[self.length as usize].assume_init_read())
            }
        }
    }

    pub unsafe fn set_len(&mut self, new_len: u8) {
        self.length = new_len;
    }

    pub unsafe fn push_unchecked(&mut self, elem: T) {
        self.array
            .get_unchecked_mut(self.length as usize)
            .write(elem);
        self.length += 1;
    }

    pub fn push(&mut self, elem: T) {
        assert!((self.length as usize) < CAP);
        unsafe { self.push_unchecked(elem) }
    }

    pub fn to_array<const LEN: usize>(self) -> [T; LEN] {
        assert!(LEN <= self.length as usize);
        unsafe {
            let original_array = self.array;

            let mut arr: [MaybeUninit<T>; LEN] = MaybeUninit::uninit_array();
            for i in 0..LEN {
                arr[i].write(original_array[i].assume_init_read());
            }

            MaybeUninit::array_assume_init(arr)
        }
    }
}

impl<T, const CAP: usize> ArrayVec<T, CAP>
where
    T: Copy,
{
    pub fn from_slice(s: &[T]) -> Self {
        assert!(CAP >= s.len());
        let mut new = Self::new();
        unsafe {
            for e in s {
                new.push_unchecked(*e);
            }
        }
        new
    }
}

impl<T, const CAP: usize> std::iter::FromIterator<T> for ArrayVec<T, CAP> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut vec = Self::new();
        for e in iter {
            vec.push(e);
        }
        vec
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod drain_filter {
    use std::ptr;
    use std::slice;

    #[derive(Debug)]
    pub struct DrainFilter<'a, T, F, const CAP: usize>
    where
        F: FnMut(&mut T) -> bool,
    {
        pub(super) vec: &'a mut crate::ArrayVec<T, CAP>,
        /// The index of the item that will be inspected by the next call to `next`.
        pub(super) idx: u8,
        /// The number of items that have been drained (removed) thus far.
        pub(super) del: u8,
        /// The original length of `vec` prior to draining.
        pub(super) old_len: u8,
        /// The filter test predicate.
        pub(super) pred: F,
        /// A flag that indicates a panic has occurred in the filter test predicate.
        /// This is used as a hint in the drop implementation to prevent consumption
        /// of the remainder of the `DrainFilter`. Any unprocessed items will be
        /// backshifted in the `vec`, but no further items will be dropped or
        /// tested by the filter predicate.
        pub(super) panic_flag: bool,
    }

    impl<T, F, const CAP: usize> Iterator for DrainFilter<'_, T, F, CAP>
    where
        F: FnMut(&mut T) -> bool,
    {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            unsafe {
                while self.idx < self.old_len {
                    let i = self.idx;
                    let v = slice::from_raw_parts_mut(self.vec.as_mut_ptr(), self.old_len as usize);
                    self.panic_flag = true;
                    let drained = (self.pred)(&mut v[i as usize]);
                    self.panic_flag = false;
                    // Update the index *after* the predicate is called. If the index
                    // is updated prior and the predicate panics, the element at this
                    // index would be leaked.
                    self.idx += 1;
                    if drained {
                        self.del += 1;
                        return Some(ptr::read(&v[i as usize]));
                    } else if self.del > 0 {
                        let del = self.del;
                        let src: *const T = &v[i as usize];
                        let dst: *mut T = &mut v[i as usize - del as usize];
                        ptr::copy_nonoverlapping(src, dst, 1);
                    }
                }
                None
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, Some(self.old_len as usize - self.idx as usize))
        }
    }

    impl<T, F, const CAP: usize> Drop for DrainFilter<'_, T, F, CAP>
    where
        F: FnMut(&mut T) -> bool,
    {
        fn drop(&mut self) {
            struct BackshiftOnDrop<'a, 'b, T, F, const CAP: usize>
            where
                F: FnMut(&mut T) -> bool,
            {
                drain: &'b mut DrainFilter<'a, T, F, CAP>,
            }

            impl<'a, 'b, T, F, const CAP: usize> Drop for BackshiftOnDrop<'a, 'b, T, F, CAP>
            where
                F: FnMut(&mut T) -> bool,
            {
                fn drop(&mut self) {
                    unsafe {
                        if self.drain.idx < self.drain.old_len && self.drain.del > 0 {
                            // This is a pretty messed up state, and there isn't really an
                            // obviously right thing to do. We don't want to keep trying
                            // to execute `pred`, so we just backshift all the unprocessed
                            // elements and tell the vec that they still exist. The backshift
                            // is required to prevent a double-drop of the last successfully
                            // drained item prior to a panic in the predicate.
                            let ptr = self.drain.vec.as_mut_ptr();
                            let src = ptr.add(self.drain.idx as usize);
                            let dst = src.sub(self.drain.del as usize);
                            let tail_len = self.drain.old_len - self.drain.idx;
                            src.copy_to(dst, tail_len as usize);
                        }
                        self.drain.vec.set_len(self.drain.old_len - self.drain.del);
                    }
                }
            }

            let backshift = BackshiftOnDrop { drain: self };

            // Attempt to consume any remaining elements if the filter predicate
            // has not yet panicked. We'll backshift any remaining elements
            // whether we've already panicked or if the consumption here panics.
            if !backshift.drain.panic_flag {
                backshift.drain.for_each(drop);
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ArrayStr<const CAP: usize>(ArrayVec<u8, CAP>);

impl<const CAP: usize> std::str::FromStr for ArrayStr<CAP> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(ArrayVec::from_slice(s.as_bytes())))
    }
}

impl<const CAP: usize> std::ops::Deref for ArrayStr<CAP> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl<const CAP: usize> std::fmt::Debug for ArrayStr<CAP> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::ops::Deref;
        self.deref().fmt(f)
    }
}

impl<const CAP: usize> std::hash::Hash for ArrayStr<CAP> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        use std::ops::Deref;
        self.deref().hash(state)
    }
}
