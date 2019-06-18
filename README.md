# CLRS-rust-implementation
[![Build Status](https://travis-ci.com/hsfzxjy/CLRS-rust-implementation.svg?branch=master)](https://travis-ci.com/hsfzxjy/CLRS-rust-implementation)

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
   + **Doubly Linked List** for `[T]` with insertion and deletion at arbitary position