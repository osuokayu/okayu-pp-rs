pub use inner::*;

#[cfg(not(feature = "raw_strains"))]
mod inner {
<<<<<<< HEAD
    use std::{
        iter::{self, Copied},
        mem,
        slice::{self, Iter},
    };

    use crate::util::hint::{likely, unlikely};
=======
    use std::{iter::Copied, slice::Iter};
>>>>>>> 42db299 (meow)

    use self::entry::StrainsEntry;

    /// A specialized `Vec<f64>` where all entries must be non-negative.
    ///
    /// It is compact in the sense that zeros are not stored directly but instead
    /// as amount of times they appear consecutively.
    ///
    /// For cases with few consecutive zeros, this type generally reduces
    /// performance slightly. However, for edge cases like `/b/3739922` the length
    /// of the list is massively reduced, preventing out-of-memory issues.
    #[derive(Clone)]
    pub struct StrainsVec {
        inner: Vec<StrainsEntry>,
        len: usize,
        #[cfg(debug_assertions)]
        // Ensures that methods are used correctly
        has_zero: bool,
    }

    impl StrainsVec {
        /// Constructs a new, empty [`StrainsVec`] with at least the specified
        /// capacity.
<<<<<<< HEAD
        #[inline]
=======
>>>>>>> 42db299 (meow)
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                inner: Vec::with_capacity(capacity),
                len: 0,
                #[cfg(debug_assertions)]
                has_zero: false,
            }
        }

        /// Returns the number of elements.
<<<<<<< HEAD
        #[inline]
=======
>>>>>>> 42db299 (meow)
        pub const fn len(&self) -> usize {
            self.len
        }

        /// Appends an element to the back.
<<<<<<< HEAD
        #[inline]
        pub fn push(&mut self, value: f64) {
            if likely(value.to_bits() > 0 && value.is_sign_positive()) {
                // SAFETY: we just checked whether it's positive
                self.inner.push(unsafe { StrainsEntry::new_value(value) });
=======
        pub fn push(&mut self, value: f64) {
            if value.to_bits() > 0 {
                self.inner.push(StrainsEntry::new_value(value));
>>>>>>> 42db299 (meow)
            } else if let Some(last) = self.inner.last_mut().filter(|e| e.is_zero()) {
                last.incr_zero_count();
            } else {
                self.inner.push(StrainsEntry::new_zero());

                #[cfg(debug_assertions)]
                {
                    self.has_zero = true;
                }
            }

            self.len += 1;
        }

        /// Sorts the entries in descending order.
<<<<<<< HEAD
        #[inline]
=======
>>>>>>> 42db299 (meow)
        pub fn sort_desc(&mut self) {
            #[cfg(debug_assertions)]
            debug_assert!(!self.has_zero);

            self.inner.sort_by(|a, b| b.value().total_cmp(&a.value()));
        }

        /// Removes all zero entries
<<<<<<< HEAD
        #[inline]
        pub fn retain_non_zero(&mut self) {
            self.inner.retain(|e| likely(e.is_value()));
=======
        pub fn retain_non_zero(&mut self) {
            self.inner.retain(StrainsEntry::is_value);
>>>>>>> 42db299 (meow)

            #[cfg(debug_assertions)]
            {
                self.has_zero = false;
            }
        }

        /// Removes all zeros and sorts the remaining entries in descending order.
<<<<<<< HEAD
        #[inline]
=======
>>>>>>> 42db299 (meow)
        pub fn retain_non_zero_and_sort(&mut self) {
            self.retain_non_zero();
            self.sort_desc();
        }

<<<<<<< HEAD
        /// Removes all zeros, sorts the remaining entries in descending order, and
        /// returns an iterator over mutable references to the values.
        #[inline]
=======
        /// Iterator over the raw entries, assuming that there are no zeros.
        ///
        /// Panics if there are zeros.
        pub fn non_zero_iter(&self) -> impl ExactSizeIterator<Item = f64> + '_ {
            #[cfg(debug_assertions)]
            debug_assert!(!self.has_zero);

            self.inner.iter().copied().map(StrainsEntry::value)
        }

        /// Same as [`StrainsVec::retain_non_zero_and_sort`] followed by
        /// [`StrainsVec::iter`] but the resulting iterator is faster
        /// because it doesn't need to check whether entries are zero.
        pub fn sorted_non_zero_iter(&mut self) -> impl ExactSizeIterator<Item = f64> + '_ {
            self.retain_non_zero_and_sort();

            self.non_zero_iter()
        }

        /// Removes all zeros, sorts the remaining entries in descending order, and
        /// returns an iterator over mutable references to the values.
>>>>>>> 42db299 (meow)
        pub fn sorted_non_zero_iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut f64> {
            self.retain_non_zero_and_sort();

            self.inner.iter_mut().map(StrainsEntry::as_value_mut)
        }

        /// Sum up all values.
<<<<<<< HEAD
        #[inline]
=======
>>>>>>> 42db299 (meow)
        pub fn sum(&self) -> f64 {
            self.inner
                .iter()
                .copied()
<<<<<<< HEAD
                .filter_map(StrainsEntry::try_as_value)
                .sum()
        }

        /// Returns an iterator over the [`StrainsVec`].
        #[inline]
=======
                .filter(StrainsEntry::is_value)
                .fold(0.0, |sum, e| sum + e.value())
        }

        /// Returns an iterator over the [`StrainsVec`].
>>>>>>> 42db299 (meow)
        pub fn iter(&self) -> StrainsIter<'_> {
            StrainsIter::new(self)
        }

<<<<<<< HEAD
        /// Converts this [`StrainsVec`] into `Vec<f64>`.
        ///
        /// # Safety
        ///
        /// `self` may not include *any* zeros.
        pub unsafe fn transmute_into_vec(self) -> Vec<f64> {
            // SAFETY: `StrainsEntry` has the same properties as `f64`
            unsafe { mem::transmute::<Vec<StrainsEntry>, Vec<f64>>(self.inner) }
        }

        /// Allocates a new `Vec<f64>` to store all values, including zeros.
        pub fn into_vec(self) -> Vec<f64> {
            /// Copies the first `count` items of `slice` into `dst`.
            fn copy_slice(slice: &[StrainsEntry], count: usize, dst: &mut Vec<f64>) {
                if unlikely(count == 0) {
                    return;
                }

                let ptr = slice.as_ptr().cast();

                // SAFETY: `StrainsEntry` has the same properties as `f64`
                let slice = unsafe { slice::from_raw_parts(ptr, count) };
                dst.extend_from_slice(slice);
            }

            /// Drives the iterator until it finds a zero count. It then copies
            /// entries up to that and returns the zero count.
            #[inline]
            fn copy_non_zero(
                iter: &mut Iter<'_, StrainsEntry>,
                dst: &mut Vec<f64>,
            ) -> Option<usize> {
                let mut count = 0;
                let slice = iter.as_slice();

                for entry in iter {
                    if unlikely(entry.is_zero()) {
                        copy_slice(slice, count, dst);

                        return Some(entry.zero_count() as usize);
                    }

                    count += 1;
                }

                copy_slice(slice, count, dst);

                None
            }

            let mut vec = Vec::with_capacity(self.len);
            let mut iter = self.inner.iter();

            while let Some(zero_count) = copy_non_zero(&mut iter, &mut vec) {
                vec.extend(iter::repeat_n(0.0, zero_count));
            }
=======
        /// Allocates a new `Vec<f64>` to store all values, including zeros.
        pub fn into_vec(self) -> Vec<f64> {
            let mut vec = Vec::with_capacity(self.len);
            vec.extend(self.iter());
>>>>>>> 42db299 (meow)

            vec
        }
    }

    pub struct StrainsIter<'a> {
        inner: Copied<Iter<'a, StrainsEntry>>,
        curr: Option<StrainsEntry>,
        len: usize,
    }

    impl<'a> StrainsIter<'a> {
        pub fn new(vec: &'a StrainsVec) -> Self {
            let mut inner = vec.inner.iter().copied();
            let curr = inner.next();

            Self {
                inner,
                curr,
                len: vec.len,
            }
        }
    }

<<<<<<< HEAD
    impl Iterator for StrainsIter<'_> {
=======
    impl<'a> Iterator for StrainsIter<'a> {
>>>>>>> 42db299 (meow)
        type Item = f64;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let curr = self.curr.as_mut()?;

<<<<<<< HEAD
                if likely(curr.is_value()) {
=======
                if curr.is_value() {
>>>>>>> 42db299 (meow)
                    let value = curr.value();
                    self.curr = self.inner.next();
                    self.len -= 1;

                    return Some(value);
                } else if curr.zero_count() > 0 {
                    curr.decr_zero_count();
                    self.len -= 1;

                    return Some(0.0);
                }

                self.curr = self.inner.next();
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            let len = self.len();

            (len, Some(len))
        }
    }

    impl ExactSizeIterator for StrainsIter<'_> {
        fn len(&self) -> usize {
            self.len
        }
    }

    /// Private module to hide internal fields.
    mod entry {
<<<<<<< HEAD
        use super::likely;

=======
>>>>>>> 42db299 (meow)
        /// Either a positive `f64` or an amount of consecutive `0.0`.
        ///
        /// If the first bit is not set, i.e. the sign bit of a `f64` indicates
        /// that it's positive, the union represents that `f64`. Otherwise, the
        /// first bit is ignored and the union represents a `u64`.
        #[derive(Copy, Clone)]
        pub union StrainsEntry {
            value: f64,
            zero_count: u64,
        }

        impl StrainsEntry {
            const ZERO_COUNT_MASK: u64 = u64::MAX >> 1;

<<<<<<< HEAD
            /// # Safety
            ///
            /// `value` must be positive, i.e. neither negative nor zero.
            #[inline]
            pub const unsafe fn new_value(value: f64) -> Self {
                Self { value }
            }

            #[inline]
=======
            pub fn new_value(value: f64) -> Self {
                debug_assert!(
                    value.is_sign_positive(),
                    "attempted to create negative strain entry, please report as a bug"
                );

                Self { value }
            }

>>>>>>> 42db299 (meow)
            pub const fn new_zero() -> Self {
                Self {
                    zero_count: !Self::ZERO_COUNT_MASK + 1,
                }
            }

<<<<<<< HEAD
            #[inline]
            pub const fn is_zero(self) -> bool {
                unsafe { self.value.is_sign_negative() }
            }

            #[inline]
            pub const fn is_value(self) -> bool {
                !self.is_zero()
            }

            #[inline]
            pub const fn value(self) -> f64 {
                unsafe { self.value }
            }

            #[inline]
            pub const fn try_as_value(self) -> Option<f64> {
                if likely(self.is_value()) {
                    Some(self.value())
                } else {
                    None
                }
            }

            #[inline]
            pub const fn as_value_mut(&mut self) -> &mut f64 {
                unsafe { &mut self.value }
            }

            #[inline]
            pub const fn zero_count(self) -> u64 {
                unsafe { self.zero_count & Self::ZERO_COUNT_MASK }
            }

            #[inline]
            pub const fn incr_zero_count(&mut self) {
=======
            pub fn is_zero(self) -> bool {
                unsafe { self.value.is_sign_negative() }
            }

            // Requiring `self` as a reference improves ergonomics for passing this
            // method as argument to higher-order functions.
            #[allow(clippy::trivially_copy_pass_by_ref)]
            pub fn is_value(&self) -> bool {
                !self.is_zero()
            }

            pub fn value(self) -> f64 {
                debug_assert!(self.is_value());

                unsafe { self.value }
            }

            pub fn as_value_mut(&mut self) -> &mut f64 {
                debug_assert!(self.is_value());

                unsafe { &mut self.value }
            }

            pub fn zero_count(self) -> u64 {
                debug_assert!(self.is_zero());

                unsafe { self.zero_count & Self::ZERO_COUNT_MASK }
            }

            pub fn incr_zero_count(&mut self) {
                debug_assert!(self.is_zero());

>>>>>>> 42db299 (meow)
                unsafe {
                    self.zero_count += 1;
                }
            }

<<<<<<< HEAD
            #[inline]
            pub const fn decr_zero_count(&mut self) {
=======
            pub fn decr_zero_count(&mut self) {
                debug_assert!(self.is_zero());

>>>>>>> 42db299 (meow)
                unsafe {
                    self.zero_count -= 1;
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use proptest::prelude::*;

        use crate::util::float_ext::FloatExt;

        use super::*;

        proptest! {
            #[test]
<<<<<<< HEAD
            fn expected(values in prop::collection::vec(prop::option::of(0.0..1_000.0), 0..1_000)) {
                let mut vec = StrainsVec::with_capacity(values.len());
                let mut raw = Vec::with_capacity(values.len());
=======
            fn expected(mut values in prop::collection::vec(prop::option::of(0.0..1_000.0), 0..1_000)) {
                let mut vec = StrainsVec::with_capacity(values.len());
>>>>>>> 42db299 (meow)

                let mut additional_zeros = 0;
                let mut prev_zero = false;
                let mut sum = 0.0;

                for opt in values.iter().copied() {
<<<<<<< HEAD
                    if let Some(value) = opt.filter(|&value| value != 0.0) {
                        let value = f64::abs(value);

                        vec.push(value);
                        raw.push(value);
=======
                    if let Some(value) = opt {
                        vec.push(value);
>>>>>>> 42db299 (meow)
                        prev_zero = false;
                        sum += value;
                    } else {
                        vec.push(0.0);
<<<<<<< HEAD
                        raw.push(0.0);
=======
>>>>>>> 42db299 (meow)

                        if prev_zero {
                            additional_zeros += 1;
                        }

                        prev_zero = true;
                    }
                }

<<<<<<< HEAD
                assert_eq!(vec.len(), raw.len());
                assert_eq!(vec.inner.len(), raw.len() - additional_zeros);
                assert!(vec.sum().eq(sum));
                assert!(vec.iter().eq(raw.iter().copied()));
                assert_eq!(vec.clone().into_vec(), raw);

                vec.retain_non_zero_and_sort();
                raw.retain(|&n| n > 0.0);
                raw.sort_by(|a, b| b.total_cmp(a));

                assert_eq!(unsafe { vec.transmute_into_vec() }, raw);
=======
                assert_eq!(vec.len(), values.len());
                assert_eq!(vec.inner.len(), values.len() - additional_zeros);
                assert!(vec.sum().eq(sum));
                assert!(vec.iter().eq(values.iter().copied().map(|opt| opt.unwrap_or(0.0))));

                values.retain(Option::is_some);

                values.sort_by(|a, b| {
                    let (Some(a), Some(b)) = (a, b) else { unreachable!() };

                    b.total_cmp(a)
                });

                assert!(vec.sorted_non_zero_iter().eq(values.into_iter().flatten()));
>>>>>>> 42db299 (meow)
            }
        }
    }
}

#[cfg(feature = "raw_strains")]
mod inner {
    use std::{
        iter::Copied,
        slice::{Iter, IterMut},
    };

    /// Plain wrapper around `Vec<f64>` because the `raw_strains` feature
    /// is disabled.
    #[derive(Clone)]
    pub struct StrainsVec {
        inner: Vec<f64>,
    }

    impl StrainsVec {
        pub fn with_capacity(capacity: usize) -> Self {
            Self {
                inner: Vec::with_capacity(capacity),
            }
        }

        pub fn len(&self) -> usize {
            self.inner.len()
        }

        pub fn push(&mut self, value: f64) {
            self.inner.push(value);
        }

        pub fn sort_desc(&mut self) {
            self.inner.sort_by(|a, b| b.total_cmp(a));
        }

        pub fn retain_non_zero(&mut self) {
            self.inner.retain(|&a| a > 0.0);
        }

        pub fn retain_non_zero_and_sort(&mut self) {
            self.retain_non_zero();
            self.sort_desc();
        }

<<<<<<< HEAD
=======
        pub fn non_zero_iter(&self) -> Copied<Iter<'_, f64>> {
            self.inner.iter().copied()
        }

        pub fn sorted_non_zero_iter(&mut self) -> Copied<Iter<'_, f64>> {
            self.retain_non_zero_and_sort();

            self.non_zero_iter()
        }

>>>>>>> 42db299 (meow)
        pub fn sorted_non_zero_iter_mut(&mut self) -> IterMut<'_, f64> {
            self.retain_non_zero_and_sort();

            self.inner.iter_mut()
        }

        pub fn sum(&self) -> f64 {
            self.inner.iter().copied().sum()
        }

        pub fn iter(&self) -> Copied<Iter<'_, f64>> {
            self.inner.iter().copied()
        }

<<<<<<< HEAD
        pub unsafe fn transmute_into_vec(self) -> Vec<f64> {
            self.inner
        }

=======
>>>>>>> 42db299 (meow)
        pub fn into_vec(self) -> Vec<f64> {
            self.inner
        }
    }
}
