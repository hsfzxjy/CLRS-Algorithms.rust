# CLRS-rust-implementation
[![Build Status](https://travis-ci.com/hsfzxjy/CLRS-Algorithms.rust.svg?branch=master)](https://travis-ci.com/hsfzxjy/CLRS-Algorithms.rust)

Rust implementation for <em>Introduction to Algorithms</em>

## Implemented

 + **Chapter 02 Getting Start**
   + **Bubble Sort** for `[PartialOrd]`
   + **Insertion Sort** for `[PartialOrd + Clone]`
   + **Merge Sort** for `[PartialOrd + Clone]`
 + **Chapter 06 Heapsort**
   + **Heap Sort** for `[PartialOrd]`
 + **Chapter 07 Quicksort**
   + **Quick Sort** for `[PartialOrd]` (Partitioned by last position)
   + **Quick Sort** for `[PartialOrd]` (Partitioned by randomized position)
 + **Chapter 08 Sorting in Linear Time**
   + **Counting Sort** for `[i32]`
   + **Radix Sort** for `[T]` where `T` is signed or unsigned primitive integer
   + **Bucket Sort** for `[T]` where `T` is bounded partial-ordered numeric type
 + **Chapter 09 Medians and Order Statistics**
   + **Randomized Order Statistics Selection** for `[PartialOrd + Copy]`, with `O(n)` time complexity in average
   + **Stable Order Statistics Selection** for `[PartialOrd + Copy]`, with `O(n)` time complexity in the worst case 
 + **Chapter 10 Elementary Data Structures**
   + **Doubly Linked List** for `T` with insertion and deletion at arbitary position
   + **Stack** for `T` using Doubly Linked List as underlying data structure
   + **Queue** for `T` using Doubly Linked List as underlying data structure
   + Efficient **BinaryTree** for `T` with
     + intuitive constructor
     + an `Anchor` struct for referencing node
     + support for replacing and detaching tree node
     + support for in/pre/post-order traverse
     + support for equality test
 + **Chapter 12 Binary Search Trees**
   + **BST** for `PartialOrd + Copy` with support for
     + insertion
     + deletion
     + maximum and minimum query from arbitary position
     + search by given key