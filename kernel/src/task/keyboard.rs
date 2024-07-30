use core::pin::Pin;
use core::task::{Context, Poll};

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::task::AtomicWaker;
use futures_util::{Stream, StreamExt};

use crate::println;
use crate::renderer::keyboard_interface::execute;

const SCANCODE_QUEUE_SIZE: usize = 100;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_err() {
            println!("[Warning] task::keyboard::add_scancode scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("[Warning] task::keyboard::add_scancode scancode queue uninitialized");
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(SCANCODE_QUEUE_SIZE))
            .expect("[Warning] task::keyboard::ScancodeStram::new should only be call once");
        ScancodeStream { _private: () }
    }
}

impl Default for ScancodeStream {
    fn default() -> Self {
        Self::new()
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("[Warning] task::keyboard::ScancodeStram::poll_next scancode queue uninitialized");

        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

pub async fn execute_keycode() {
    let mut scancodes = ScancodeStream::new();
    while let Some(scancode) = scancodes.next().await {
        execute(scancode);
    }
}
