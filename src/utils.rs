// SPDX-License-Identifier: MPIT

//! Utility functions for handling data in this library.

use crate::reexports::iced_core::{
    layout::{Limits, Node},
    Point, Size,
};

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

/// Produces a [`Node`] with two children nodes one right next to each other.
pub fn next_to_each_other(
    limits: &Limits,
    spacing: f32,
    left: impl FnOnce(&Limits) -> Node,
    right: impl FnOnce(&Limits) -> Node,
) -> Node {
    let mut right_node = right(limits);
    let right_size = right_node.size();

    let left_limits = limits.shrink(Size::new(right_size.width + spacing, 0.0));
    let mut left_node = left(&left_limits);
    let left_size = left_node.size();

    let (left_y, right_y) = if left_size.height > right_size.height {
        (0.0, (left_size.height - right_size.height) / 2.0)
    } else {
        ((right_size.height - left_size.height) / 2.0, 0.0)
    };

    left_node = left_node.move_to(Point::new(0.0, left_y));
    right_node = right_node.move_to(Point::new(left_size.width + spacing, right_y));

    Node::with_children(
        Size::new(
            left_size.width + spacing + right_size.width,
            left_size.height.max(right_size.height),
        ),
        vec![left_node, right_node],
    )
}
