# mut-binary-heap

![Rust](https://github.com/sekineh/binary-heap-plus-rs/workflows/Rust/badge.svg)

Enhancement over Rust's
[`std::collections::BinaryHeap`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html).

The important change to it's original implementation is the addition of the `update_key` method,
which allows for updating the position of existing entries in the heap.

Other changes to the standard library implementation are support for the following heaps:
- Max heap
  - Use `BinaryHeap::new()` or `::with_capacity()`
- Min heap
  - Use `BinaryHeap::new_min()` or `::with_capacity_min()`
- Heap ordered by closure
  - Use `BinaryHeap::new_by()` or `::with_capacity_by()`
- Heap ordered by key generated by closure
  - Use `BinaryHeap::new_by_key()` or `::with_capacity_by_key()`

## Compatibility and MSRV (Minimum Supported Rust Version)

This crate is based on [binary-heap-plus](https://github.com/sekineh/binary-heap-plus-rs),
which is based on the standard library's implementation of
[`BinaryHeap`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html)
from Rust 1.62.0.

The minimum supported Rust version is 1.56.0.

# Changes

See
[CHANGELOG.md](https://github.com/Wasabi375/binary-heap-plus-rs/blob/master/CHANGELOG.md).

