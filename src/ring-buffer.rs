extern crate nix;
extern crate ring_buffer;
extern crate shared_memory;

use std::ffi::OsStr;
use std::process;
use std::time;

use nix::unistd::{fork, ForkResult};
use shared_memory::{LockType, SharedMem, WriteLockable};

use ring_buffer::{LENGTH, RingBuffer};

//2**20
const MAX_LEN: usize = 1_048_576;

fn main() {
  let mut num = 0;

  let mut shared_data =
    SharedMem::create_linked(OsStr::new("shared_mem.link"),
                             LockType::Mutex,
                             LENGTH * 8 + 16)
      .unwrap_or_else(|e| panic!("Shared memory create error: {}", e));

  {
    let mut shared_state =
      shared_data.wlock::<RingBuffer>(0)
                 .unwrap_or_else(|e| panic!("Failed to create write lock {}", e));

    shared_state.data = [0usize; LENGTH];
    shared_state.start_idx = 0;
    shared_state.end_idx = 0;
  }

  println!("Producer initialized");

  let start_time = time::Instant::now();
  match fork() {
    Ok(ForkResult::Parent { .. }) => {
      // Producer
      loop {
        let mut shared_state =
          shared_data.wlock::<RingBuffer>(0)
                     .unwrap_or_else(|e| panic!("Read lock error {}", e));

        if shared_state.is_empty() && num == MAX_LEN {
          let end_time = time::Instant::now();
          let elapsed = end_time.duration_since(start_time).subsec_nanos() as u64 / 1_000_000;
          let elapsed_sec = end_time.duration_since(start_time).as_secs() * 1_000;
          println!("Duration {:?} (ms)", elapsed_sec + elapsed);
          process::exit(0)
        } else if num < MAX_LEN {
          if shared_state.insert(num + 1) {
            num += 1;
          }
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
        // TODO: ensure that something is done with data

        drop(rb);
      }
    }
    Err(e) => panic!("Failed to fork child process {}", e),
  }
}
