// <数据流的中位数>

// Median is the middle value in an ordered integer list. If the size of the list is even, there is no middle value. So the median is the mean of the two middle value.

// For example,
// [2,3,4], the median is 3
// [2,3], the median is (2 + 3) / 2 = 2.5

// Design a data structure that supports the following two operations:
// void addNum(int num) - Add a integer number from the data stream to the data structure.
// double findMedian() - Return the median of all elements so far.

// Example:
// addNum(1)
// addNum(2)
// findMedian() -> 1.5
// addNum(3)
// findMedian() -> 2

// TODO Follow up:
// If all integer numbers from the stream are between 0 and 100, how would you optimize it?
// If 99% of all integer numbers from the stream are between 0 and 100, how would you optimize it?

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/**
 * 大顶堆 + 小顶堆
 * 大顶堆存储较小的值，小顶堆存储较大的值
 */
#[derive(Debug)]
struct MedianFinder {
    big_heap: BinaryHeap<i32>,
    small_heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    /** initialize your data structure here. */
    fn new() -> Self {
        MedianFinder {
            big_heap: BinaryHeap::new(),
            small_heap: BinaryHeap::new(),
        }
    }

    /**
     * 根据大小判断num插入哪个堆，并始终保持两个堆的平衡(元素差不超过1)
     */
    fn add_num(&mut self, num: i32) {
        if !self.small_heap.is_empty() {
            if num >= self.small_heap.peek().unwrap().0 {
                self.small_heap.push(Reverse(num));
                if self.small_heap.len() - self.big_heap.len() > 1 {
                    self.big_heap.push(self.small_heap.pop().unwrap().0);
                }
            } else {
                self.big_heap.push(num);
                if self.big_heap.len() > self.small_heap.len() {
                    self.small_heap.push(Reverse(self.big_heap.pop().unwrap()));
                }
            }
        } else {
            self.small_heap.push(Reverse(num));
        }
    }

    /**
     * 大小堆大小一样时，取两个堆的堆顶求中位数
     * 否则取元素较多堆的堆顶为中位数
     */
    fn find_median(&self) -> f64 {
        if !self.small_heap.is_empty() {
            if self.small_heap.len() == self.big_heap.len() {
                return (self.big_heap.peek().unwrap() + self.small_heap.peek().unwrap().0) as f64
                    / 2.0;
            } else {
                return self.small_heap.peek().unwrap().0 as f64;
            }
        } else {
            return 0.0;
        }
    }
}

#[test]
fn run() {
    let mut finder = MedianFinder::new();
    finder.add_num(1);
    finder.add_num(2);
    assert_eq!(finder.find_median(), 1.5 as f64);
    finder.add_num(3);
    assert_eq!(finder.find_median(), 2 as f64);
}
