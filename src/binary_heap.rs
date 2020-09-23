// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A priority queue implemented with a binary heap.
//!
//! Note: This version is folked from Rust standartd library, which only supports
//! max heap.
//!
//! Insertion and popping the largest element have `O(log n)` time complexity.
//! Checking the largest element is `O(1)`. Converting a vector to a binary heap
//! can be done in-place, and has `O(n)` complexity. A binary heap can also be
//! converted to a sorted vector in-place, allowing it to be used for an `O(n
//! log n)` in-place heapsort.
//!
//! # Examples
//!
//! This is a larger example that implements [Dijkstra's algorithm][dijkstra]
//! to solve the [shortest path problem][sssp] on a [directed graph][dir_graph].
//! It shows how to use [`BinaryHeap`] with custom types.
//!
//! [dijkstra]: http://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
//! [sssp]: http://en.wikipedia.org/wiki/Shortest_path_problem
//! [dir_graph]: http://en.wikipedia.org/wiki/Directed_graph
//! [`BinaryHeap`]: struct.BinaryHeap.html
//!
//! ```
//! use std::cmp::Ordering;
//! use binary_heap_plus::*;
//! use std::usize;
//!
//! #[derive(Copy, Clone, Eq, PartialEq)]
//! struct State {
//!     cost: usize,
//!     position: usize,
//! }
//!
//! // The priority queue depends on `Ord`.
//! // Explicitly implement the trait so the queue becomes a min-heap
//! // instead of a max-heap.
//! impl Ord for State {
//!     fn cmp(&self, other: &State) -> Ordering {
//!         // Notice that the we flip the ordering on costs.
//!         // In case of a tie we compare positions - this step is necessary
//!         // to make implementations of `PartialEq` and `Ord` consistent.
//!         other.cost.cmp(&self.cost)
//!             .then_with(|| self.position.cmp(&other.position))
//!     }
//! }
//!
//! // `PartialOrd` needs to be implemented as well.
//! impl PartialOrd for State {
//!     fn partial_cmp(&self, other: &State) -> Option<Ordering> {
//!         Some(self.cmp(other))
//!     }
//! }
//!
//! // Each node is represented as an `usize`, for a shorter implementation.
//! struct Edge {
//!     node: usize,
//!     cost: usize,
//! }
//!
//! // Dijkstra's shortest path algorithm.
//!
//! // Start at `start` and use `dist` to track the current shortest distance
//! // to each node. This implementation isn't memory-efficient as it may leave duplicate
//! // nodes in the queue. It also uses `usize::MAX` as a sentinel value,
//! // for a simpler implementation.
//! fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
//!     // dist[node] = current shortest distance from `start` to `node`
//!     let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
//!
//!     let mut heap = BinaryHeap::new();
//!
//!     // We're at `start`, with a zero cost
//!     dist[start] = 0;
//!     heap.push(State { cost: 0, position: start });
//!
//!     // Examine the frontier with lower cost nodes first (min-heap)
//!     while let Some(State { cost, position }) = heap.pop() {
//!         // Alternatively we could have continued to find all shortest paths
//!         if position == goal { return Some(cost); }
//!
//!         // Important as we may have already found a better way
//!         if cost > dist[position] { continue; }
//!
//!         // For each node we can reach, see if we can find a way with
//!         // a lower cost going through this node
//!         for edge in &adj_list[position] {
//!             let next = State { cost: cost + edge.cost, position: edge.node };
//!
//!             // If so, add it to the frontier and continue
//!             if next.cost < dist[next.position] {
//!                 heap.push(next);
//!                 // Relaxation, we have now found a better way
//!                 dist[next.position] = next.cost;
//!             }
//!         }
//!     }
//!
//!     // Goal not reachable
//!     None
//! }
//!
//! fn main() {
//!     // This is the directed graph we're going to use.
//!     // The node numbers correspond to the different states,
//!     // and the edge weights symbolize the cost of moving
//!     // from one node to another.
//!     // Note that the edges are one-way.
//!     //
//!     //                  7
//!     //          +-----------------+
//!     //          |                 |
//!     //          v   1        2    |  2
//!     //          0 -----> 1 -----> 3 ---> 4
//!     //          |        ^        ^      ^
//!     //          |        | 1      |      |
//!     //          |        |        | 3    | 1
//!     //          +------> 2 -------+      |
//!     //           10      |               |
//!     //                   +---------------+
//!     //
//!     // The graph is represented as an adjacency list where each index,
//!     // corresponding to a node value, has a list of outgoing edges.
//!     // Chosen for its efficiency.
//!     let graph = vec![
//!         // Node 0
//!         vec![Edge { node: 2, cost: 10 },
//!              Edge { node: 1, cost: 1 }],
//!         // Node 1
//!         vec![Edge { node: 3, cost: 2 }],
//!         // Node 2
//!         vec![Edge { node: 1, cost: 1 },
//!              Edge { node: 3, cost: 3 },
//!              Edge { node: 4, cost: 1 }],
//!         // Node 3
//!         vec![Edge { node: 0, cost: 7 },
//!              Edge { node: 4, cost: 2 }],
//!         // Node 4
//!         vec![]];
//!
//!     assert_eq!(shortest_path(&graph, 0, 1), Some(1));
//!     assert_eq!(shortest_path(&graph, 0, 3), Some(3));
//!     assert_eq!(shortest_path(&graph, 3, 0), Some(7));
//!     assert_eq!(shortest_path(&graph, 0, 4), Some(5));
//!     assert_eq!(shortest_path(&graph, 4, 0), None);
//! }
//! ```

#![allow(missing_docs)]
// #![stable(feature = "rust1", since = "1.0.0")]

// use core::ops::{Deref, DerefMut, Place, Placer, InPlace};
// use core::iter::{FromIterator, FusedIterator};
use std::cmp::Ordering;
use std::iter::FromIterator;
use std::slice;
// use std::iter::FusedIterator;
// use std::vec::Drain;
use compare::Compare;
use core::fmt;
use core::mem::{size_of, swap};
use core::ptr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::ops::DerefMut;
use std::vec;

// use slice;
// use vec::{self, Vec};

// use super::SpecExtend;

/// A priority queue implemented with a binary heap.
///
/// This will be a max-heap.
///
/// It is a logic error for an item to be modified in such a way that the
/// item's ordering relative to any other item, as determined by the `Ord`
/// trait, changes while it is in the heap. This is normally only possible
/// through `Cell`, `RefCell`, global state, I/O, or unsafe code.
///
/// # Examples
///
/// ```
/// use binary_heap_plus::*;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `BinaryHeap<i32, MaxComparator>` in this example).
/// let mut heap = BinaryHeap::new();
///
/// // We can use peek to look at the next item in the heap. In this case,
/// // there's no items in there yet so we get None.
/// assert_eq!(heap.peek(), None);
///
/// // Let's add some scores...
/// heap.push(1);
/// heap.push(5);
/// heap.push(2);
///
/// // Now peek shows the most important item in the heap.
/// assert_eq!(heap.peek(), Some(&5));
///
/// // We can check the length of a heap.
/// assert_eq!(heap.len(), 3);
///
/// // We can iterate over the items in the heap, although they are returned in
/// // a random order.
/// for x in &heap {
///     println!("{}", x);
/// }
///
/// // If we instead pop these scores, they should come back in order.
/// assert_eq!(heap.pop(), Some(5));
/// assert_eq!(heap.pop(), Some(2));
/// assert_eq!(heap.pop(), Some(1));
/// assert_eq!(heap.pop(), None);
///
/// // We can clear the heap of any remaining items.
/// heap.clear();
///
/// // The heap should now be empty.
/// assert!(heap.is_empty())
/// ```
// #[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BinaryHeap<T, C = MaxComparator>
where
    C: Compare<T>,
{
    data: Vec<T>,
    cmp: C,
}

/// For `T` that implements `Ord`, you can use this struct to quickly
/// set up a max heap.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct MaxComparator;

impl<T: Ord> Compare<T> for MaxComparator {
    fn compare(&self, a: &T, b: &T) -> Ordering {
        a.cmp(&b)
    }
}

/// For `T` that implements `Ord`, you can use this struct to quickly
/// set up a min heap.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct MinComparator;

impl<T: Ord> Compare<T> for MinComparator {
    fn compare(&self, a: &T, b: &T) -> Ordering {
        b.cmp(&a)
    }
}

/// The comparator defined by closure
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct FnComparator<F>(pub F);

impl<T, F> Compare<T> for FnComparator<F>
where
    F: Fn(&T, &T) -> Ordering,
{
    fn compare(&self, a: &T, b: &T) -> Ordering {
        self.0(a, b)
    }
}

/// The comparator ordered by key
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct KeyComparator<F>(pub F);

impl<K: Ord, T, F> Compare<T> for KeyComparator<F>
where
    F: Fn(&T) -> K,
{
    fn compare(&self, a: &T, b: &T) -> Ordering {
        self.0(a).cmp(&self.0(b))
    }
}

/// Structure wrapping a mutable reference to the greatest item on a
/// `BinaryHeap`.
///
/// This `struct` is created by the [`peek_mut`] method on [`BinaryHeap`]. See
/// its documentation for more.
///
/// [`peek_mut`]: struct.BinaryHeap.html#method.peek_mut
/// [`BinaryHeap`]: struct.BinaryHeap.html
// #[stable(feature = "binary_heap_peek_mut", since = "1.12.0")]
pub struct PeekMut<'a, T: 'a, C: 'a + Compare<T>> {
    heap: &'a mut BinaryHeap<T, C>,
    sift: bool,
}

// #[stable(feature = "collection_debug", since = "1.17.0")]
impl<'a, T: fmt::Debug, C: Compare<T>> fmt::Debug for PeekMut<'a, T, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("PeekMut").field(&self.heap.data[0]).finish()
    }
}

// #[stable(feature = "binary_heap_peek_mut", since = "1.12.0")]
impl<'a, T, C: Compare<T>> Drop for PeekMut<'a, T, C> {
    fn drop(&mut self) {
        if self.sift {
            self.heap.sift_down(0);
        }
    }
}

// #[stable(feature = "binary_heap_peek_mut", since = "1.12.0")]
impl<'a, T, C: Compare<T>> Deref for PeekMut<'a, T, C> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.heap.data[0]
    }
}

// #[stable(feature = "binary_heap_peek_mut", since = "1.12.0")]
impl<'a, T, C: Compare<T>> DerefMut for PeekMut<'a, T, C> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.heap.data[0]
    }
}

impl<'a, T, C: Compare<T>> PeekMut<'a, T, C> {
    /// Removes the peeked value from the heap and returns it.
    // #[stable(feature = "binary_heap_peek_mut_pop", since = "1.18.0")]
    pub fn pop(mut this: PeekMut<'a, T, C>) -> T {
        let value = this.heap.pop().unwrap();
        this.sift = false;
        value
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<T: Clone, C: Compare<T> + Clone> Clone for BinaryHeap<T, C> {
    fn clone(&self) -> Self {
        BinaryHeap {
            data: self.data.clone(),
            cmp: self.cmp.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<T: Ord> Default for BinaryHeap<T> {
    /// Creates an empty `BinaryHeap<T>`.
    #[inline]
    fn default() -> BinaryHeap<T> {
        BinaryHeap::new()
    }
}

// #[stable(feature = "binaryheap_debug", since = "1.4.0")]
impl<T: fmt::Debug, C: Compare<T>> fmt::Debug for BinaryHeap<T, C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T, C: Compare<T> + Default> BinaryHeap<T, C> {
    /// Generic constructor for `BinaryHeap` from `Vec`.
    ///
    /// Because `BinaryHeap` stores the elements in its internal `Vec`,
    /// it's natural to construct it from `Vec`.
    pub fn from_vec(vec: Vec<T>) -> Self {
        BinaryHeap::from_vec_cmp(vec, C::default())
    }
}

impl<T, C: Compare<T>> BinaryHeap<T, C> {
    /// Generic constructor for `BinaryHeap` from `Vec` and comparator.
    ///
    /// Because `BinaryHeap` stores the elements in its internal `Vec`,
    /// it's natural to construct it from `Vec`.
    pub fn from_vec_cmp(vec: Vec<T>, cmp: C) -> Self {
        unsafe { BinaryHeap::from_vec_cmp_raw(vec, cmp, true) }
    }

    /// Generic constructor for `BinaryHeap` from `Vec` and comparator.
    ///
    /// Because `BinaryHeap` stores the elements in its internal `Vec`,
    /// it's natural to construct it from `Vec`.
    ///
    /// # Safety
    /// User is responsible for providing valid `rebuild` value.
    pub unsafe fn from_vec_cmp_raw(vec: Vec<T>, cmp: C, rebuild: bool) -> Self {
        let mut heap = BinaryHeap { data: vec, cmp };
        if rebuild && !heap.data.is_empty() {
            heap.rebuild();
        }
        heap
    }
}

impl<T: Ord> BinaryHeap<T> {
    /// Creates an empty `BinaryHeap`.
    ///
    /// This default version will create a max-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::new();
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(5));
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn new() -> Self {
        BinaryHeap::from_vec(vec![])
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// This default version will create a max-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::with_capacity(10);
    /// assert_eq!(heap.capacity(), 10);
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(5));
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn with_capacity(capacity: usize) -> Self {
        BinaryHeap::from_vec(Vec::with_capacity(capacity))
    }
}

impl<T: Ord> BinaryHeap<T, MinComparator> {
    /// Creates an empty `BinaryHeap`.
    ///
    /// The `_min()` version will create a min-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::new_min();
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    pub fn new_min() -> Self {
        BinaryHeap::from_vec(vec![])
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// The `_min()` version will create a min-heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::with_capacity_min(10);
    /// assert_eq!(heap.capacity(), 10);
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    pub fn with_capacity_min(capacity: usize) -> Self {
        BinaryHeap::from_vec(Vec::with_capacity(capacity))
    }
}

impl<T, F> BinaryHeap<T, FnComparator<F>>
where
    F: Fn(&T, &T) -> Ordering,
{
    /// Creates an empty `BinaryHeap`.
    ///
    /// The `_by()` version will create a heap ordered by given closure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::new_by(|a: &i32, b: &i32| b.cmp(a));
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    pub fn new_by(f: F) -> Self {
        BinaryHeap::from_vec_cmp(vec![], FnComparator(f))
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// The `_by()` version will create a heap ordered by given closure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::with_capacity_by(10, |a: &i32, b: &i32| b.cmp(a));
    /// assert_eq!(heap.capacity(), 10);
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    pub fn with_capacity_by(capacity: usize, f: F) -> Self {
        BinaryHeap::from_vec_cmp(Vec::with_capacity(capacity), FnComparator(f))
    }
}

impl<T, F, K: Ord> BinaryHeap<T, KeyComparator<F>>
where
    F: Fn(&T) -> K,
{
    /// Creates an empty `BinaryHeap`.
    ///
    /// The `_by_key()` version will create a heap ordered by key converted by given closure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::new_by_key(|a: &i32| a % 4);
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(3));
    /// ```
    pub fn new_by_key(f: F) -> Self {
        BinaryHeap::from_vec_cmp(vec![], KeyComparator(f))
    }

    /// Creates an empty `BinaryHeap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `BinaryHeap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// The `_by_key()` version will create a heap ordered by key coverted by given closure.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::with_capacity_by_key(10, |a: &i32| a % 4);
    /// assert_eq!(heap.capacity(), 10);
    /// heap.push(3);
    /// heap.push(1);
    /// heap.push(5);
    /// assert_eq!(heap.pop(), Some(3));
    /// ```
    pub fn with_capacity_by_key(capacity: usize, f: F) -> Self {
        BinaryHeap::from_vec_cmp(Vec::with_capacity(capacity), KeyComparator(f))
    }
}

impl<T, C: Compare<T>> BinaryHeap<T, C> {
    /// Returns an iterator visiting all values in the underlying vector, in
    /// arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let heap = BinaryHeap::from(vec![1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.iter() {
    ///     println!("{}", x);
    /// }
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn iter(&self) -> Iter<T> {
        Iter {
            iter: self.data.iter(),
        }
    }

    /// Returns an iterator which retrieves elements in heap order.
    /// This method consumes the original heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let heap = BinaryHeap::from(vec![1, 2, 3, 4, 5]);
    ///
    /// assert_eq!(heap.into_iter_sorted().take(2).collect::<Vec<_>>(), vec![5, 4]);
    /// ```
    // #[unstable(feature = "binary_heap_into_iter_sorted", issue = "59278")]
    pub fn into_iter_sorted(self) -> IntoIterSorted<T, C> {
        IntoIterSorted {
            inner: self,
        }
    }

    /// Returns the greatest item in the binary heap, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::new();
    /// assert_eq!(heap.peek(), None);
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// assert_eq!(heap.peek(), Some(&5));
    ///
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    /// Returns a mutable reference to the greatest item in the binary heap, or
    /// `None` if it is empty.
    ///
    /// Note: If the `PeekMut` value is leaked, the heap may be in an
    /// inconsistent state.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::new();
    /// assert!(heap.peek_mut().is_none());
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    /// {
    ///     let mut val = heap.peek_mut().unwrap();
    ///     *val = 0;
    /// }
    /// assert_eq!(heap.peek(), Some(&2));
    /// ```
    // #[stable(feature = "binary_heap_peek_mut", since = "1.12.0")]
    pub fn peek_mut(&mut self) -> Option<PeekMut<T, C>> {
        if self.is_empty() {
            None
        } else {
            Some(PeekMut {
                heap: self,
                sift: true,
            })
        }
    }

    /// Returns the number of elements the binary heap can hold without reallocating.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::with_capacity(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Reserves the minimum capacity for exactly `additional` more elements to be inserted in the
    /// given `BinaryHeap`. Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it requests. Therefore
    /// capacity can not be relied upon to be precisely minimal. Prefer [`reserve`] if future
    /// insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::new();
    /// heap.reserve_exact(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    ///
    /// [`reserve`]: #method.reserve
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the
    /// `BinaryHeap`. The collection may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::new();
    /// heap.reserve(100);
    /// assert!(heap.capacity() >= 100);
    /// heap.push(4);
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Discards as much additional capacity as possible.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap: BinaryHeap<i32> = BinaryHeap::with_capacity(100);
    ///
    /// assert!(heap.capacity() >= 100);
    /// heap.shrink_to_fit();
    /// assert!(heap.capacity() == 0);
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Removes the greatest item from the binary heap and returns it, or `None` if it
    /// is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::from(vec![1, 3]);
    ///
    /// assert_eq!(heap.pop(), Some(3));
    /// assert_eq!(heap.pop(), Some(1));
    /// assert_eq!(heap.pop(), None);
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop().map(|mut item| {
            if !self.is_empty() {
                swap(&mut item, &mut self.data[0]);
                self.sift_down_to_bottom(0);
            }
            item
        })
    }

    /// Pushes an item onto the binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::new();
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert_eq!(heap.len(), 3);
    /// assert_eq!(heap.peek(), Some(&5));
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn push(&mut self, item: T) {
        let old_len = self.len();
        self.data.push(item);
        self.sift_up(0, old_len);
    }

    /// Consumes the `BinaryHeap` and returns the underlying vector
    /// in arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let heap = BinaryHeap::from(vec![1, 2, 3, 4, 5, 6, 7]);
    /// let vec = heap.into_vec();
    ///
    /// // Will print in some order
    /// for x in vec {
    ///     println!("{}", x);
    /// }
    /// ```
    // #[stable(feature = "binary_heap_extras_15", since = "1.5.0")]
    pub fn into_vec(self) -> Vec<T> {
        self.into()
    }

    /// Consumes the `BinaryHeap` and returns a vector in sorted
    /// (ascending) order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    ///
    /// let mut heap = BinaryHeap::from(vec![1, 2, 4, 5, 7]);
    /// heap.push(6);
    /// heap.push(3);
    ///
    /// let vec = heap.into_sorted_vec();
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    // #[stable(feature = "binary_heap_extras_15", since = "1.5.0")]
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        let mut end = self.len();
        while end > 1 {
            end -= 1;
            self.data.swap(0, end);
            self.sift_down_range(0, end);
        }
        self.into_vec()
    }

    // The implementations of sift_up and sift_down use unsafe blocks in
    // order to move an element out of the vector (leaving behind a
    // hole), shift along the others and move the removed element back into the
    // vector at the final location of the hole.
    // The `Hole` type is used to represent this, and make sure
    // the hole is filled back at the end of its scope, even on panic.
    // Using a hole reduces the constant factor compared to using swaps,
    // which involves twice as many moves.
    fn sift_up(&mut self, start: usize, pos: usize) -> usize {
        unsafe {
            // Take out the value at `pos` and create a hole.
            let mut hole = Hole::new(&mut self.data, pos);

            while hole.pos() > start {
                let parent = (hole.pos() - 1) / 2;
                // if hole.element() <= hole.get(parent) {
                if self.cmp.compare(hole.element(), hole.get(parent)) != Ordering::Greater {
                    break;
                }
                hole.move_to(parent);
            }
            hole.pos()
        }
    }

    /// Take an element at `pos` and move it down the heap,
    /// while its children are larger.
    fn sift_down_range(&mut self, pos: usize, end: usize) {
        unsafe {
            let mut hole = Hole::new(&mut self.data, pos);
            let mut child = 2 * pos + 1;
            while child < end {
                let right = child + 1;
                // compare with the greater of the two children
                // if right < end && !(hole.get(child) > hole.get(right)) {
                if right < end
                    && self.cmp.compare(hole.get(child), hole.get(right)) != Ordering::Greater
                {
                    child = right;
                }
                // if we are already in order, stop.
                // if hole.element() >= hole.get(child) {
                if self.cmp.compare(hole.element(), hole.get(child)) != Ordering::Less {
                    break;
                }
                hole.move_to(child);
                child = 2 * hole.pos() + 1;
            }
        }
    }

    fn sift_down(&mut self, pos: usize) {
        let len = self.len();
        self.sift_down_range(pos, len);
    }

    /// Take an element at `pos` and move it all the way down the heap,
    /// then sift it up to its position.
    ///
    /// Note: This is faster when the element is known to be large / should
    /// be closer to the bottom.
    fn sift_down_to_bottom(&mut self, mut pos: usize) {
        let end = self.len();
        let start = pos;
        unsafe {
            let mut hole = Hole::new(&mut self.data, pos);
            let mut child = 2 * pos + 1;
            while child < end {
                let right = child + 1;
                // compare with the greater of the two children
                // if right < end && !(hole.get(child) > hole.get(right)) {
                if right < end
                    && self.cmp.compare(hole.get(child), hole.get(right)) != Ordering::Greater
                {
                    child = right;
                }
                hole.move_to(child);
                child = 2 * hole.pos() + 1;
            }
            pos = hole.pos;
        }
        self.sift_up(start, pos);
    }

    /// Returns the length of the binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let heap = BinaryHeap::from(vec![1, 3]);
    ///
    /// assert_eq!(heap.len(), 2);
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the binary heap is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::new();
    ///
    /// assert!(heap.is_empty());
    ///
    /// heap.push(3);
    /// heap.push(5);
    /// heap.push(1);
    ///
    /// assert!(!heap.is_empty());
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the binary heap, returning an iterator over the removed elements.
    ///
    /// The elements are removed in arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::from(vec![1, 3]);
    ///
    /// assert!(!heap.is_empty());
    ///
    /// for x in heap.drain() {
    ///     println!("{}", x);
    /// }
    ///
    /// assert!(heap.is_empty());
    /// ```
    #[inline]
    // #[stable(feature = "drain", since = "1.6.0")]
    pub fn drain(&mut self) -> Drain<T> {
        Drain {
            iter: self.data.drain(..),
        }
    }

    /// Drops all items from the binary heap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let mut heap = BinaryHeap::from(vec![1, 3]);
    ///
    /// assert!(!heap.is_empty());
    ///
    /// heap.clear();
    ///
    /// assert!(heap.is_empty());
    /// ```
    // #[stable(feature = "rust1", since = "1.0.0")]
    pub fn clear(&mut self) {
        self.drain();
    }

    fn rebuild(&mut self) {
        let mut n = self.len() / 2;
        while n > 0 {
            n -= 1;
            self.sift_down(n);
        }
    }

    /// Moves all the elements of `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    ///
    /// let v = vec![-10, 1, 2, 3, 3];
    /// let mut a = BinaryHeap::from(v);
    ///
    /// let v = vec![-20, 5, 43];
    /// let mut b = BinaryHeap::from(v);
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    /// assert!(b.is_empty());
    /// ```
    // #[stable(feature = "binary_heap_append", since = "1.11.0")]
    pub fn append(&mut self, other: &mut Self) {
        if self.len() < other.len() {
            swap(self, other);
        }

        if other.is_empty() {
            return;
        }

        #[inline(always)]
        fn log2_fast(x: usize) -> usize {
            8 * size_of::<usize>() - (x.leading_zeros() as usize) - 1
        }

        // `rebuild` takes O(len1 + len2) operations
        // and about 2 * (len1 + len2) comparisons in the worst case
        // while `extend` takes O(len2 * log_2(len1)) operations
        // and about 1 * len2 * log_2(len1) comparisons in the worst case,
        // assuming len1 >= len2.
        #[inline]
        fn better_to_rebuild(len1: usize, len2: usize) -> bool {
            2 * (len1 + len2) < len2 * log2_fast(len1)
        }

        if better_to_rebuild(self.len(), other.len()) {
            self.data.append(&mut other.data);
            self.rebuild();
        } else {
            self.extend(other.drain());
        }
    }
}

/// Hole represents a hole in a slice i.e. an index without valid value
/// (because it was moved from or duplicated).
/// In drop, `Hole` will restore the slice by filling the hole
/// position with the value that was originally removed.
struct Hole<'a, T: 'a> {
    data: &'a mut [T],
    /// `elt` is always `Some` from new until drop.
    elt: Option<T>,
    pos: usize,
}

impl<'a, T> Hole<'a, T> {
    /// Create a new Hole at index `pos`.
    ///
    /// Unsafe because pos must be within the data slice.
    #[inline]
    unsafe fn new(data: &'a mut [T], pos: usize) -> Self {
        debug_assert!(pos < data.len());
        let elt = ptr::read(&data[pos]);
        Hole {
            data,
            elt: Some(elt),
            pos,
        }
    }

    #[inline]
    fn pos(&self) -> usize {
        self.pos
    }

    /// Returns a reference to the element removed.
    #[inline]
    fn element(&self) -> &T {
        self.elt.as_ref().unwrap()
    }

    /// Returns a reference to the element at `index`.
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn get(&self, index: usize) -> &T {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());
        self.data.get_unchecked(index)
    }

    /// Move hole to new location
    ///
    /// Unsafe because index must be within the data slice and not equal to pos.
    #[inline]
    unsafe fn move_to(&mut self, index: usize) {
        debug_assert!(index != self.pos);
        debug_assert!(index < self.data.len());
        let index_ptr: *const _ = self.data.get_unchecked(index);
        let hole_ptr = self.data.get_unchecked_mut(self.pos);
        ptr::copy_nonoverlapping(index_ptr, hole_ptr, 1);
        self.pos = index;
    }
}

impl<'a, T> Drop for Hole<'a, T> {
    #[inline]
    fn drop(&mut self) {
        // fill the hole again
        unsafe {
            let pos = self.pos;
            ptr::write(self.data.get_unchecked_mut(pos), self.elt.take().unwrap());
        }
    }
}

/// An iterator over the elements of a `BinaryHeap`.
///
/// This `struct` is created by the [`iter`] method on [`BinaryHeap`]. See its
/// documentation for more.
///
/// [`iter`]: struct.BinaryHeap.html#method.iter
/// [`BinaryHeap`]: struct.BinaryHeap.html
// #[stable(feature = "rust1", since = "1.0.0")]
pub struct Iter<'a, T: 'a> {
    iter: slice::Iter<'a, T>,
}

// #[stable(feature = "collection_debug", since = "1.17.0")]
impl<'a, T: 'a + fmt::Debug> fmt::Debug for Iter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Iter").field(&self.iter.as_slice()).finish()
    }
}

// FIXME(#26925) Remove in favor of `#[derive(Clone)]`
// #[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Iter<'a, T> {
        Iter {
            iter: self.iter.clone(),
        }
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        self.iter.next_back()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<'a, T> ExactSizeIterator for Iter<'a, T> {
//     fn is_empty(&self) -> bool {
//         self.iter.is_empty()
//     }
// }

// #[stable(feature = "fused", since = "1.26.0")]
// impl<'a, T> FusedIterator for Iter<'a, T> {}

/// An owning iterator over the elements of a `BinaryHeap`.
///
/// This `struct` is created by the [`into_iter`] method on [`BinaryHeap`][`BinaryHeap`]
/// (provided by the `IntoIterator` trait). See its documentation for more.
///
/// [`into_iter`]: struct.BinaryHeap.html#method.into_iter
/// [`BinaryHeap`]: struct.BinaryHeap.html
// #[stable(feature = "rust1", since = "1.0.0")]
#[derive(Clone)]
pub struct IntoIter<T> {
    iter: vec::IntoIter<T>,
}

// #[stable(feature = "collection_debug", since = "1.17.0")]
impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("IntoIter")
            .field(&self.iter.as_slice())
            .finish()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
// impl<T> ExactSizeIterator for IntoIter<T> {
//     fn is_empty(&self) -> bool {
//         self.iter.is_empty()
//     }
// }

// #[stable(feature = "fused", since = "1.26.0")]
// impl<T> FusedIterator for IntoIter<T> {}

// #[unstable(feature = "binary_heap_into_iter_sorted", issue = "59278")]
#[derive(Clone, Debug)]
pub struct IntoIterSorted<T, C: Compare<T>> {
    inner: BinaryHeap<T, C>,
}

// #[unstable(feature = "binary_heap_into_iter_sorted", issue = "59278")]
impl<T, C: Compare<T>> Iterator for IntoIterSorted<T, C> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.inner.pop()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.inner.len();
        (exact, Some(exact))
    }
}

/// A draining iterator over the elements of a `BinaryHeap`.
///
/// This `struct` is created by the [`drain`] method on [`BinaryHeap`]. See its
/// documentation for more.
///
/// [`drain`]: struct.BinaryHeap.html#method.drain
/// [`BinaryHeap`]: struct.BinaryHeap.html
// #[stable(feature = "drain", since = "1.6.0")]
// #[derive(Debug)]
pub struct Drain<'a, T: 'a> {
    iter: vec::Drain<'a, T>,
}

// #[stable(feature = "drain", since = "1.6.0")]
impl<'a, T: 'a> Iterator for Drain<'a, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

// #[stable(feature = "drain", since = "1.6.0")]
impl<'a, T: 'a> DoubleEndedIterator for Drain<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

// #[stable(feature = "drain", since = "1.6.0")]
// impl<'a, T: 'a> ExactSizeIterator for Drain<'a, T> {
//     fn is_empty(&self) -> bool {
//         self.iter.is_empty()
//     }
// }

// #[stable(feature = "fused", since = "1.26.0")]
// impl<'a, T: 'a> FusedIterator for Drain<'a, T> {}

// #[stable(feature = "binary_heap_extras_15", since = "1.5.0")]
impl<T: Ord> From<Vec<T>> for BinaryHeap<T> {
    /// creates a max heap from a vec
    fn from(vec: Vec<T>) -> Self {
        BinaryHeap::from_vec(vec)
    }
}

// #[stable(feature = "binary_heap_extras_15", since = "1.5.0")]
// impl<T, C: Compare<T>> From<BinaryHeap<T, C>> for Vec<T> {
//     fn from(heap: BinaryHeap<T, C>) -> Vec<T> {
//         heap.data
//     }
// }

impl<T, C: Compare<T>> Into<Vec<T>> for BinaryHeap<T, C> {
    fn into(self) -> Vec<T> {
        self.data
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<T: Ord> FromIterator<T> for BinaryHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        BinaryHeap::from(iter.into_iter().collect::<Vec<_>>())
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<T, C: Compare<T>> IntoIterator for BinaryHeap<T, C> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the binary heap in arbitrary order. The binary heap cannot be used
    /// after calling this.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use binary_heap_plus::*;
    /// let heap = BinaryHeap::from(vec![1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in heap.into_iter() {
    ///     // x has type i32, not &i32
    ///     println!("{}", x);
    /// }
    /// ```
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            iter: self.data.into_iter(),
        }
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<'a, T, C: Compare<T>> IntoIterator for &'a BinaryHeap<T, C> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

// #[stable(feature = "rust1", since = "1.0.0")]
impl<T, C: Compare<T>> Extend<T> for BinaryHeap<T, C> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        // <Self as SpecExtend<I>>::spec_extend(self, iter);
        self.extend_desugared(iter);
    }
}

// impl<T, I: IntoIterator<Item = T>> SpecExtend<I> for BinaryHeap<T> {
//     default fn spec_extend(&mut self, iter: I) {
//         self.extend_desugared(iter.into_iter());
//     }
// }

// impl<T> SpecExtend<BinaryHeap<T>> for BinaryHeap<T> {
//     fn spec_extend(&mut self, ref mut other: BinaryHeap<T>) {
//         self.append(other);
//     }
// }

impl<T, C: Compare<T>> BinaryHeap<T, C> {
    fn extend_desugared<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let iterator = iter.into_iter();
        let (lower, _) = iterator.size_hint();

        self.reserve(lower);

        for elem in iterator {
            self.push(elem);
        }
    }
}

// #[stable(feature = "extend_ref", since = "1.2.0")]
impl<'a, T: 'a + Copy, C: Compare<T>> Extend<&'a T> for BinaryHeap<T, C> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// pub struct BinaryHeapPlace<'a, T: 'a>
// where T: Clone {
//     heap: *mut BinaryHeap<T>,
//     place: vec::PlaceBack<'a, T>,
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T: Clone + Ord + fmt::Debug> fmt::Debug for BinaryHeapPlace<'a, T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         f.debug_tuple("BinaryHeapPlace")
//          .field(&self.place)
//          .finish()
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T: 'a> Placer<T> for &'a mut BinaryHeap<T>
// where T: Clone + Ord {
//     type Place = BinaryHeapPlace<'a, T>;

//     fn make_place(self) -> Self::Place {
//         let ptr = self as *mut BinaryHeap<T>;
//         let place = Placer::make_place(self.data.place_back());
//         BinaryHeapPlace {
//             heap: ptr,
//             place,
//         }
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// unsafe impl<'a, T> Place<T> for BinaryHeapPlace<'a, T>
// where T: Clone + Ord {
//     fn pointer(&mut self) -> *mut T {
//         self.place.pointer()
//     }
// }

// #[unstable(feature = "collection_placement",
//            reason = "placement protocol is subject to change",
//            issue = "30172")]
// impl<'a, T> InPlace<T> for BinaryHeapPlace<'a, T>
// where T: Clone + Ord {
//     type Owner = &'a T;

//     unsafe fn finalize(self) -> &'a T {
//         self.place.finalize();

//         let heap: &mut BinaryHeap<T> = &mut *self.heap;
//         let len = heap.len();
//         let i = heap.sift_up(0, len - 1);
//         heap.data.get_unchecked(i)
//     }
// }
