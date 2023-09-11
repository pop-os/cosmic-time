// SPDX-License-Identifier: MPIT

//! Utility functions for handling data in this library.

/// Collect iterator into static array without panicking or collecting into a Vec.
///
/// Initializes with `T::default()`, then takes `SIZE` values from the iterator.
pub fn static_array_from_iter<T: Copy + Default, const SIZE: usize>(
    iter: impl Iterator<Item = T>,
) -> [T; SIZE] {
    let mut array = [T::default(); SIZE];

    for (id, value) in iter.take(SIZE).enumerate() {
        array[id] = value;
    }

    array
}
