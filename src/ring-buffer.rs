extern crate nix;
extern crate ring_buffer;
extern crate shared_memory;

use nix::unistd::{fork, ForkResult};
use shared_memory::{LockType, SharedMem, WriteLockable};
use std::ffi::OsStr;
use std::process;
use ring_buffer::{LENGTH, RingBuffer};

fn main() {
  let mut num = 0;

  let mut shared_data =
    SharedMem::create_linked(OsStr::new("shared_mem.link"),
                             LockType::Mutex,
                             16384)
      .unwrap_or_else(|e| panic!("Shared memory create error: {}", e));

  {
    let mut shared_state =
      shared_data.wlock::<RingBuffer>(0)
        .unwrap_or_else(|e| panic!("Failed to create write lock {}", e));

    shared_state.data = [0usize; LENGTH];
    shared_state.start_idx = 0;
    shared_state.end_idx = 0;
    shared_state.size = 0;
  }

  println!("Producer initialized");

  match fork() {
    Ok(ForkResult::Parent { .. }) => {
      // Producer
      loop {
        let mut shared_state =
          shared_data.wlock::<RingBuffer>(0)
            .unwrap_or_else(|e| panic!("Read lock error {}", e));

        shared_state.insert(num);

        num += 1;
        //2**24
        if num == 16_777_216 {
          process::exit(0)
        }

        drop(shared_state);
      }
    }
    Ok(ForkResult::Child) => {
      // Consumer
      loop {
        let mut rb = shared_data.wlock::<RingBuffer>(0)
          .unwrap_or_else(|e| panic!("Failed to get read lock {}", e));

        rb.pop();

        drop(rb);
      }
    }
    Err(e) => panic!("Failed to fork child process {:?}", e),
  }
}
