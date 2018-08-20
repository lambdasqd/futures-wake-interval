#![feature(arbitrary_self_types, futures_api, pin)]

use std::future::{Future, FutureObj};
use std::mem::PinMut;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

/// State shared between the future and the timer thread
struct SharedState {
    waker: Option<Waker>,
    completed: bool,
}

impl SharedState {
    fn new() -> Self {
        SharedState {
            waker: None,
            completed: false,
        }
    }
}

/// Wrapper that periodically wakes a future
///
/// This can be useful for futures that have a ready condition that needs to be checked periodically
/// because no other event that would trigger a wake.
pub struct WakeInterval<'a, T> {
    first: bool,                           // `true` on the first call to `poll()`
    interval: Duration,                    // time to wait between wakes
    future: FutureObj<'a, T>,              // inner future that we want to wake and poll
    shared_state: Arc<Mutex<SharedState>>, // shared with the timer thread
}

impl<'a, T> WakeInterval<'a, T> {
    pub fn new(interval: Duration, future: FutureObj<'a, T>) -> Self {
        WakeInterval {
            first: true,
            interval,
            future,
            shared_state: Arc::new(Mutex::new(SharedState::new())),
        }
    }
}

impl<'a, T> Future for WakeInterval<'a, T> {
    type Output = T;

    fn poll(mut self: PinMut<Self>, cx: &mut Context) -> Poll<<Self as Future>::Output> {
        // The waker may change between calls to `poll()` so we must update the shared_state
        {
            let mut shared_state = self.shared_state.lock().unwrap();
            shared_state.waker = Some(cx.waker().clone());
        }

        // Creates a new thread that will act as a timer to wake the task
        if self.first {
            self.first = false;

            // These will be moved into the thread
            let duration = self.interval.clone();
            let shared_state = self.shared_state.clone();

            thread::spawn(move || loop {
                thread::sleep(duration);
                let shared_state = shared_state.lock().unwrap();
                if shared_state.completed {
                    return;
                }
                if let Some(ref waker) = shared_state.waker {
                    waker.wake();
                }
            });
        }

        // Poll the inner future
        match PinMut::new(&mut self.future).poll(cx) {
            Poll::Ready(val) => {
                // Signal the looping thread that we are done
                let mut shared_state = self.shared_state.lock().unwrap();
                shared_state.completed = true;
                Poll::Ready(val)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
