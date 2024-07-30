use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::task::Wake;
use core::task::{Context, Poll, Waker};

use crossbeam_queue::ArrayQueue;

use super::{Task, TaskId};

const TASK_QUEUE_SIZE: usize = 100;

pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

pub struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    #[allow(clippy::new_ret_no_self)]
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker { task_id, task_queue }))
    }

    pub fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("Task queue full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }
}

impl Executor {
    pub fn new() -> Executor {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(TASK_QUEUE_SIZE)),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id();
        if self.tasks.insert(task_id, task).is_some() {
            panic!("Task with same ID already in tasks");
        }
        self.task_queue.push(task_id).expect("Task queue full");
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    fn run_ready_tasks(&mut self) {
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Some(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue,
            };

            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));

            let waker = waker.clone();
            let context = &mut Context::from_waker(&waker);

            match task.poll(context) {
                Poll::Ready(()) => {
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }

    fn sleep_if_idle(&self) {
        use x86_64::instructions::interrupts::{self, enable_and_hlt};
        interrupts::disable();
        if self.task_queue.is_empty() {
            enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}
