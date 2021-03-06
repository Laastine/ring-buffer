extern crate shared_memory;

use shared_memory::SharedMemCast;

pub const LENGTH: usize = 1024;

//#[derive(Debug)]
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

  pub fn is_empty(&self) -> bool {
    self.start_idx == self.end_idx
  }

  pub fn insert(&mut self, el: usize) -> bool {
    if self.is_full() {
      return false
    }
    if self.end_idx < LENGTH - 1 {
      self.data[self.end_idx] = el;
      self.end_idx += 1;
    } else {
      self.data[self.end_idx] = el;
      self.end_idx = 0;
    }
    true
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

#[test]
fn is_full_test() {
  let full = RingBuffer::new([0; LENGTH], 6, 5);
  let full2 = RingBuffer::new([0; LENGTH], 0, LENGTH - 1);
  let full3 = RingBuffer::new([0; LENGTH], 1, 0);
  let mut full4 = RingBuffer::new([0; LENGTH], 2, 1);

  let not_full = RingBuffer::new([0; LENGTH], 0, 0);
  let not_full2 = RingBuffer::new([0; LENGTH], 4, LENGTH - 1);
  let not_full3 = RingBuffer::new([0; LENGTH], LENGTH - 1, 1);
  let mut not_full4 = RingBuffer::new([0; LENGTH], 1, LENGTH - 1);

  assert_eq!(true, full2.is_full());
  assert_eq!(true, full.is_full());
  assert_eq!(true, full3.is_full());
  assert_eq!(true, full4.is_full());

  let insert_full4_res = full4.insert(12);
  assert_eq!(false, insert_full4_res);
  assert_eq!(true, full4.is_full());

  let pop_res = full4.pop();
  assert_eq!(Some(0), pop_res);
  assert_eq!(false, full4.is_full());

  assert_eq!(false, not_full.is_full());
  assert_eq!(false, not_full2.is_full());
  assert_eq!(false, not_full3.is_full());

  let insert_not_full4_res = not_full4.insert(11);
  assert_eq!(true, insert_not_full4_res);
  assert_eq!(true, not_full4.is_full());
}
