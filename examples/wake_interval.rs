#![feature(arbitrary_self_types, futures_api, pin)]

extern crate futures;
extern crate futures_wake_interval;

use futures::executor::ThreadPool;
use futures_wake_interval::WakeInterval;
use std::future::Future;
use std::mem::PinMut;
use std::task::{Context, Poll};
use std::time::Duration;
/// Toy future that is ready when it is polled `ready_count` number of times
struct WaitForPoll {
    num_polls: usize,   // number of times polled so far
    ready_count: usize, // ready after this many polls
}

impl WaitForPoll {
    fn new(ready_count: usize) -> Self {
        WaitForPoll {
            num_polls: 0,
            ready_count,
        }
    }
}

/// Toy future that is ready when it is polled `ready_count` number of times
impl Future for WaitForPoll {
    type Output = ();

    /// Increments the `num_polls` and is ready when ready_count is reached
    fn poll(mut self: PinMut<Self>, _cx: &mut Context) -> Poll<<Self as Future>::Output> {
        self.num_polls += 1;
        println!("poll {}", self.num_polls);

        if self.num_polls == self.ready_count {
            return Poll::Ready(());
        }
        Poll::Pending
    }
}

fn main() {
    let ready_count = 3;
    println!("future will return after {} polls", ready_count);

    let fut = Box::new(WaitForPoll::new(ready_count));
    let mut wake = WakeInterval::new(Duration::from_millis(1000), fut.into());
    let wake = PinMut::new(&mut wake);
    ThreadPool::new().unwrap().run(wake);
}
