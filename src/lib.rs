extern crate shared_memory;

use shared_memory::SharedMemCast;

pub const LENGTH: usize = 8;

#[derive(Debug)]
pub struct RingBuffer {
  pub data: [usize; LENGTH],
  pub start_idx: usize,
  pub end_idx: usize,
}

impl RingBuffer {
  pub fn new(data: [usize; LENGTH],
             start_idx: usize,
             end_idx: usize) -> Self {
    RingBuffer {
      data,
      start_idx,
      end_idx,
    }
  }

  fn is_full(&self) -> bool {
    ((self.end_idx + 1) % LENGTH) == (self.start_idx % LENGTH)
  }

  fn is_empty(&self) -> bool {
    self.start_idx == self.end_idx
  }

  pub fn insert(&mut self, el: usize) -> () {
    if self.is_full() {
      self.start_idx = if self.start_idx == LENGTH - 1 { 0 } else { self.start_idx + 1 };
    }
    if self.end_idx < LENGTH - 1 {
      self.data[self.end_idx] = el;
      self.end_idx += 1;
    } else {
      self.data[self.end_idx] = el;
      self.end_idx = 0;
    }
  }

  pub fn pop(&mut self) -> Option<usize> {
    if self.is_empty() {
      return None;
    }
    let el = self.data[self.start_idx];
    if self.start_idx == LENGTH - 1 {
      self.start_idx = 0;
    } else {
      self.start_idx += 1;
    }
    Some(el)
  }
}

unsafe impl SharedMemCast for RingBuffer {}
