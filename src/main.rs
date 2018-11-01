pub const LENGTH: usize = 8;


#[derive(Debug)]
struct RingBuffer {
  pub data: [usize; LENGTH],
  pub size: usize,
  pub start_idx: usize,
  pub end_idx: usize,
}

impl RingBuffer {
  pub fn new() -> Self {
    RingBuffer {
      data: [0usize; LENGTH],
      size: 0,
      start_idx: 0,
      end_idx: 0,
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
    println!("start_idx: {}, size: {}", self.start_idx, self.size);
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

fn main() {
  let mut rb = RingBuffer::new();
  rb.insert(0);
//  println!("{:?}", rb);

  rb.insert(1);
//  println!("{:?}", rb);
  rb.insert(2);
//  println!("{:?}", rb);
  rb.insert(3);
//  println!("{:?}", rb);
  rb.insert(4);
//  println!("{:?}", rb);
  rb.insert(5);
//  println!("{:?}", rb);
  rb.insert(6);
//  println!("{:?}", rb);
  rb.insert(7);
//  println!("{:?}", rb);
  rb.insert(8);
//  println!("{:?}", rb);
  rb.insert(9);
//  println!("{:?}", rb);
  rb.insert(10);
//  println!("{:?}", rb);
  rb.insert(11);
//  println!("{:?}", rb);
  rb.insert(12);
//  println!("{:?}", rb);
  rb.insert(13);
//  println!("{:?}", rb);
  rb.insert(14);
//  println!("{:?}", rb);
  rb.insert(15);
  rb.insert(16);
  rb.pop();
  let res = rb.pop();

  println!("Hello, world {:?}, {}", rb, res);
}
