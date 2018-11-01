extern crate ring_buffer;
extern crate shared_memory;

use shared_memory::{LockType, SharedMem, WriteLockable};
use std::ffi::OsStr;
use std::thread::sleep;
use std::time::Duration;
use ring_buffer::{LENGTH, RingBuffer};

pub fn pop(rbuf: &mut RingBuffer) -> usize {
  if rbuf.size == 0 {
    panic!("Empty ring buffer")
  }
  let el = rbuf.data[rbuf.start_idx];
  if rbuf.start_idx == LENGTH - 1 {
    rbuf.start_idx = 0;
  } else {
    rbuf.start_idx += 1;
  }
  rbuf.size -= 1;
  el
}

fn main() {
  let mut shared_data = match SharedMem::open_linked(OsStr::new("shared_mem.link")) {
    Ok(val) => val,
    Err(e) => panic!("Shared memory open error: {}", e)
  };

  loop {
    let mut rb = match shared_data.wlock::<RingBuffer>(0) {
      Ok(val) => val,
      Err(e) => panic!("Failed to get read lock {}", e),
    };

    let el = pop(&mut rb);

    println!("Popped {}", el);

    drop(rb);
    sleep(Duration::from_millis(1000));
  }
}
