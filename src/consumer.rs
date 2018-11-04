extern crate ring_buffer;
extern crate shared_memory;

use shared_memory::{SharedMem, WriteLockable};
use std::ffi::OsStr;
use std::thread::sleep;
use std::time::Duration;
use ring_buffer::RingBuffer;

fn main() {
  let mut shared_data = SharedMem::open_linked(OsStr::new("shared_mem.link"))
    .unwrap_or_else(|e| panic!("Shared memory open error: {}", e));

  loop {
    let mut rb = shared_data.wlock::<RingBuffer>(0)
      .unwrap_or_else(|e| panic!("Failed to get read lock {}", e));

    let el = rb.pop();

    println!("Popped {}", el);

    drop(rb);
    sleep(Duration::from_nanos(1));
  }
}