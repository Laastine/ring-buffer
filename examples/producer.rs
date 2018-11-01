extern crate ring_buffer;
extern crate shared_memory;

use shared_memory::{LockType, ReadLockable, SharedMem, WriteLockable};
use std::ffi::OsStr;
use std::thread::sleep;
use std::time::Duration;
use ring_buffer::{LENGTH, RingBuffer};

pub fn insert(rbuf: &mut RingBuffer, el: usize) -> () {
  if rbuf.size >= LENGTH {
    rbuf.start_idx = if rbuf.start_idx == LENGTH - 1 { 0 } else { rbuf.start_idx + 1 };
  }
  if rbuf.end_idx < LENGTH - 1 {
    rbuf.data[rbuf.end_idx] = el;
    rbuf.end_idx += 1;
  } else {
    rbuf.data[rbuf.end_idx] = el;
    rbuf.end_idx = 0;
  }
  if rbuf.size < LENGTH { rbuf.size += 1; }
}

fn main() {
  let mut num = 0;

  let mut shared_data = SharedMem::create_linked(OsStr::new("shared_mem.link"),
                                                 LockType::Mutex,
                                                 4096)
    .unwrap_or_else(|e| panic!("Shared memory create error: {}", e));


  {
    let mut shared_state = match shared_data.wlock::<RingBuffer>(0) {
      Ok(val) => val,
      Err(e) => panic!("Failed to create write lock {}", e),
    };

    shared_state.data = [0usize; LENGTH];
    shared_state.start_idx = 0;
    shared_state.end_idx = 0;
    shared_state.size = 0;
  }

  println!("Producer initialized");

  loop {
    let mut shared_state = match shared_data.wlock::<RingBuffer>(0) {
      Ok(val) => val,
      Err(e) => panic!("Read lock error {}", e),
    };

    insert(&mut shared_state, num);
    num += 1;

    println!("Data {:?}", shared_state.data);
    drop(shared_state);
    sleep(Duration::from_millis(500));
  }
}
