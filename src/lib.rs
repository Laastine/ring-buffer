extern crate shared_memory;

use shared_memory::{SharedMemCast};

pub const LENGTH: usize = 32;

#[derive(Debug)]
pub struct RingBuffer {
  pub data: [usize; LENGTH],
  pub size: usize,
  pub start_idx: usize,
  pub end_idx: usize,
}

impl RingBuffer {
  pub fn new_empty() -> Self {
    RingBuffer {
      data: [0usize; LENGTH],
      size: 0,
      start_idx: 0,
      end_idx: 0,
    }
  }

  pub fn new(data: [usize; LENGTH],
             size: usize,
             start_idx: usize,
             end_idx: usize) -> Self {
    RingBuffer {
      data,
      size,
      start_idx,
      end_idx,
    }
  }

  pub fn insert(&mut self, el: usize) -> () {
    if self.size >= LENGTH {
      self.start_idx = if self.start_idx == LENGTH - 1 { 0 } else { self.start_idx + 1 };
    }
    if self.end_idx < LENGTH - 1 {
      self.data[self.end_idx] = el;
      self.end_idx += 1;
    } else {
      self.data[self.end_idx] = el;
      self.end_idx = 0;
    }
    if self.size < LENGTH { self.size += 1; }
//    println!("start_idx: {}, size: {}", self.start_idx, self.size);
  }

  pub fn pop(&mut self) -> usize {
    if self.size == 0 {
      panic!("Empty ring buffer")
    }
    let el = self.data[self.start_idx];
    if self.start_idx == LENGTH - 1 {
      self.start_idx = 0;
    } else {
      self.start_idx += 1;
    }
    self.size -= 1;
    el
  }
}

unsafe impl SharedMemCast for RingBuffer {}
