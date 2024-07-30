pub mod executor;
pub mod keyboard;

use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::AtomicU64;
use core::sync::atomic::Ordering::Relaxed;
use core::task::{Context, Poll};

use executor::Executor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Relaxed))
    }
}

pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }

    pub fn poll(&mut self, cx: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(cx)
    }

    pub fn id(&self) -> TaskId {
        self.id
    }
}

pub fn init_executor() {
    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::execute_keycode()));
    executor.run();
}
